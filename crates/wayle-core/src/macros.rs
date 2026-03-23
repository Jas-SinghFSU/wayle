/// Merges `.watch()` streams from multiple [`Property`](crate::Property) fields
/// into a single stream that clones and emits the parent struct whenever
/// any field changes.
///
/// Typically used to give a service struct a single `watch()` method.
///
/// ```rust,ignore
/// impl AudioDevice {
///     pub fn watch(&self) -> impl Stream<Item = Self> + Send {
///         watch_all!(self, volume, muted, name)
///     }
/// }
/// ```
#[macro_export]
macro_rules! watch_all {
    ($self:expr, $($source:ident),+ $(,)?) => {
        {
            use ::futures::StreamExt;

            let cloned = $self.clone();
            let streams: Vec<::futures::stream::BoxStream<'_, ()>> = vec![
                $($self.$source.watch().map(|_| ()).boxed(),)+
            ];
            ::futures::stream::select_all(streams).map(move |_| cloned.clone())
        }
    };
}

/// Extracts a value from a D-Bus property `Result`, returning
/// [`Default::default()`] on error and logging the failure at
/// `debug` level.
///
/// Pass an optional object path as the second argument to include
/// it in the log message for easier debugging.
///
/// ```rust,ignore
/// let name: String = unwrap_dbus!(proxy.name().await);
/// let mtu: u32 = unwrap_dbus!(proxy.mtu().await, device_path);
/// ```
#[macro_export]
macro_rules! unwrap_dbus {
    ($result:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::debug!("cannot fetch property: {}", err);
            Default::default()
        })
    };

    ($result:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::debug!("cannot fetch property for {:?}: {}", $path, err);
            Default::default()
        })
    };
}

/// Like [`unwrap_dbus!`] but returns a specific fallback value
/// instead of [`Default::default()`].
///
/// ```rust,ignore
/// let cycles: i32 = unwrap_dbus_or!(proxy.charge_cycles().await, -1);
/// let scan: i64 = unwrap_dbus_or!(proxy.last_scan().await, device_path, -1);
/// ```
#[macro_export]
macro_rules! unwrap_dbus_or {
    ($result:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::debug!("cannot fetch property: {}", err);
            $default
        })
    };

    ($result:expr, $path:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::debug!("cannot fetch property for {:?}: {}", $path, err);
            $default
        })
    };
}

/// Removes items from a [`Property<Vec<T>>`](crate::Property) whose
/// `object_path` matches `$target_path`, cancelling their
/// `CancellationToken` before removal.
///
/// Items in the vec must have both an `object_path: OwnedObjectPath`
/// field and a `cancellation_token: Option<CancellationToken>` field.
///
/// ```rust,ignore
/// // Stop monitoring and remove a bluetooth device that disappeared.
/// remove_and_cancel!(self.devices, removed_path);
/// ```
#[macro_export]
macro_rules! remove_and_cancel {
    ($property:expr, $target_path:expr) => {{
        let mut items = $property.get();
        items.retain(|item| {
            if item.object_path != $target_path {
                return true;
            }

            if let Some(token) = item.cancellation_token.as_ref() {
                token.cancel();
            }

            false
        });
        $property.set(items);
    }};
}
