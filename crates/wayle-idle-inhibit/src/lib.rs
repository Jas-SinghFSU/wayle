//! Wayland idle inhibitor for GTK4 applications.
//!
//! Prevents the compositor from blanking/dimming/locking the display while
//! an [`IdleInhibitor`] exists and its associated surface is visible.
//!
//! # Example
//!
//! ```ignore
//! use wayle_idle_inhibit::IdleInhibitor;
//!
//! let surface = window.surface();
//! let inhibitor = IdleInhibitor::new(&surface);
//! // Display stays awake while `inhibitor` exists
//! drop(inhibitor);
//! // And not any more
//! ```
//!
//! # Linking Note (gtk4-layer-shell)
//!
//! If your binary also uses `gtk4-layer-shell`, it must be linked **before**
//! `libwayland-client` so its symbol interposition works. This crate declares
//! Wayland externs but does not force link order. Ensure the final binary
//! controls link order (via a build script that links `gtk4-layer-shell`
//! before `wayland-client`).
//!
//! Minimal build script example for a binary crate:
//!
//! ```ignore
//! // build.rs
//! fn main() {
//!     println!("cargo:rerun-if-changed=build.rs");
//!     println!("cargo:rustc-link-lib=gtk4-layer-shell");
//!     println!("cargo:rustc-link-lib=wayland-client");
//! }
//! ```

mod ffi;

use std::{
    ptr::{self, NonNull},
    sync::OnceLock,
};

use glib::object::{ObjectExt, ObjectType};
use tracing::{debug, error};

use crate::ffi::{
    REGISTRY_LISTENER, RegistryState, WlDisplay, WlProxy, create_inhibitor, destroy_inhibitor,
    gdk_wayland_display_get_wl_display, gdk_wayland_surface_get_wl_surface,
    wl_display_get_registry, wl_display_roundtrip, wl_proxy_add_listener,
};

// === === === === === === === === === ===
// ===           Public API            ===
// === === === === === === === === === ===

/// Wayland idle inhibitor.
///
/// Prevents the compositor from blanking/dimming/locking the display due to
/// inactivity while this exists and the associated surface is visible.
pub struct IdleInhibitor {
    ptr: *mut ffi::Inhibitor,
}

/// # Safety
///
/// The inhibitor is bound to a Wayland surface, not thread-local state.
/// The underlying protocol object is reference-counted by the compositor.
unsafe impl Send for IdleInhibitor {}

impl IdleInhibitor {
    /// Creates an inhibitor for the given GDK surface.
    ///
    /// Returns `None` if not on Wayland, compositor lacks idle-inhibit support,
    /// or the surface is not a Wayland surface.
    pub fn new(surface: &gdk4::Surface) -> Option<Self> {
        let mgr = manager()?;

        if !surface.type_().name().starts_with("GdkWayland") {
            error!(surface_type = %surface.type_().name(), "not a Wayland surface");
            return None;
        }

        let wl_surface = get_wl_surface(surface).or_else(|| {
            error!("gdk_wayland_surface_get_wl_surface returned null");
            None
        })?;

        // SAFETY: mgr and wl_surface are valid.
        let inhibitor = NonNull::new(unsafe { create_inhibitor(mgr, wl_surface.as_ptr()) })
            .or_else(|| {
                error!("create_inhibitor returned null");
                None
            })?;

        debug!("idle inhibitor created");

        Some(Self {
            ptr: inhibitor.as_ptr(),
        })
    }
}

impl Drop for IdleInhibitor {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        // SAFETY: ptr was created by create_inhibitor and hasn't been destroyed.
        unsafe { destroy_inhibitor(self.ptr) };
        debug!("idle inhibitor released");
    }
}

// === === === === === === === === === ===
// ===         Global Manager          ===
// === === === === === === === === === ===

static MANAGER: OnceLock<Option<ManagerPtr>> = OnceLock::new();

struct ManagerPtr(*mut ffi::IdleInhibitManager);

/// # Safety
///
/// The pointer is obtained from `wl_registry.bind` and remains valid for the
/// application lifetime. Wayland protocol handles reference counting.
unsafe impl Send for ManagerPtr {}
unsafe impl Sync for ManagerPtr {}

fn bind_manager() -> Option<*mut ffi::IdleInhibitManager> {
    if !ffi::is_available() {
        debug!("wayland-client symbols not available");
        return None;
    }

    let display = gdk4::Display::default()?;
    if display.type_().name() != "GdkWaylandDisplay" {
        debug!("not a Wayland display");
        return None;
    }

    let wl_display = get_wl_display(&display).or_else(|| {
        error!("gdk_wayland_display_get_wl_display returned null");
        None
    })?;

    // SAFETY: wl_display is valid.
    let registry =
        NonNull::new(unsafe { wl_display_get_registry(wl_display.as_ptr()) }).or_else(|| {
            error!("wl_display_get_registry returned null");
            None
        })?;

    let mut state = RegistryState {
        manager: ptr::null_mut(),
    };

    // SAFETY: Valid registry, listener struct, and state pointer.
    unsafe {
        wl_proxy_add_listener(
            registry.as_ptr() as *mut WlProxy,
            &REGISTRY_LISTENER as *const _ as *const _,
            &mut state as *mut _ as *mut _,
        );
    }

    // SAFETY: Blocking roundtrip invokes registry callbacks.
    if unsafe { wl_display_roundtrip(wl_display.as_ptr()) } < 0 {
        error!("wl_display_roundtrip failed");
        return None;
    }

    NonNull::new(state.manager)
        .map(|ptr| {
            debug!(ptr = ?ptr.as_ptr(), "idle_inhibit_manager bound");
            ptr.as_ptr()
        })
        .or_else(|| {
            error!("zwp_idle_inhibit_manager_v1 not available");
            None
        })
}

fn manager() -> Option<*mut ffi::IdleInhibitManager> {
    MANAGER
        .get_or_init(|| bind_manager().map(ManagerPtr))
        .as_ref()
        .map(|m| m.0)
}

// === === === === === === === === === ===
// ===           GDK Helpers           ===
// === === === === === === === === === ===

fn get_wl_display(gdk_display: &gdk4::Display) -> Option<NonNull<WlDisplay>> {
    // SAFETY: Caller verified this is a GdkWaylandDisplay.
    let ptr = unsafe {
        gdk_wayland_display_get_wl_display(gdk_display.as_ptr() as *mut ffi::GdkWaylandDisplay)
    };
    NonNull::new(ptr)
}

fn get_wl_surface(gdk_surface: &gdk4::Surface) -> Option<NonNull<ffi::WlSurface>> {
    // SAFETY: Caller verified this is a GdkWaylandSurface.
    let ptr = unsafe {
        gdk_wayland_surface_get_wl_surface(gdk_surface.as_ptr() as *mut ffi::GdkWaylandSurface)
    };
    NonNull::new(ptr)
}
