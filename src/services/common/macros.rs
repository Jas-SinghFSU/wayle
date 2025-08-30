/// Creates a watch method that combines multiple stream sources into one.
///
/// This macro provides a consistent watch API for types that need to aggregate
/// multiple change streams. It takes any fields that have a `watch()` method
/// and combines them into a single stream that emits the full struct whenever
/// any field changes.
///
/// # Example
/// ```ignore
/// impl MyStruct {
///     pub fn watch(&self) -> impl Stream<Item = Self> + Send {
///         watch_all!(self, field1, field2, field3)
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

/// Unwraps a DBus string property with empty string default.
#[macro_export]
macro_rules! unwrap_string {
    ($result:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch property: {}", err);
            String::new()
        })
    };
    ($result:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch property for {:?}: {}", $path, err);
            String::new()
        })
    };
}

/// Unwraps a DBus string property with custom default.
#[macro_export]
macro_rules! unwrap_string_or {
    ($result:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch property: {}", err);
            $default
        })
    };
    ($result:expr, $path:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch property for {:?}: {}", $path, err);
            $default
        })
    };
}

/// Unwraps a DBus boolean property with false default.
#[macro_export]
macro_rules! unwrap_bool {
    ($result:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch property: {}", err);
            false
        })
    };
    ($result:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch property for {:?}: {}", $path, err);
            false
        })
    };
}

/// Unwraps a DBus boolean property with custom default.
#[macro_export]
macro_rules! unwrap_bool_or {
    ($result:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            $default
        })
    };
    ($result:expr, $path:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            $default
        })
    };
}

/// Unwraps a DBus u8 property with 0 default.
#[macro_export]
macro_rules! unwrap_u8 {
    ($result:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            0u8
        })
    };
    ($result:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            0u8
        })
    };
}

/// Unwraps a DBus u8 property with custom default.
#[macro_export]
macro_rules! unwrap_u8_or {
    ($result:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            $default
        })
    };
    ($result:expr, $path:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            $default
        })
    };
}

/// Unwraps a DBus u16 property with 0 default.
#[macro_export]
macro_rules! unwrap_u16 {
    ($result:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            0u16
        })
    };
    ($result:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            0u16
        })
    };
}

/// Unwraps a DBus u16 property with custom default.
#[macro_export]
macro_rules! unwrap_u16_or {
    ($result:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            $default
        })
    };
    ($result:expr, $path:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            $default
        })
    };
}

/// Unwraps a DBus u64 property with 0 default.
#[macro_export]
macro_rules! unwrap_u64 {
    ($result:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            0u64
        })
    };
    ($result:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            0u64
        })
    };
}

/// Unwraps a DBus u64 property with custom default.
#[macro_export]
macro_rules! unwrap_u64_or {
    ($result:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            $default
        })
    };
    ($result:expr, $path:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            $default
        })
    };
}

/// Unwraps a DBus i64 property with 0 default.
#[macro_export]
macro_rules! unwrap_i64 {
    ($result:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            0i64
        })
    };
    ($result:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            0i64
        })
    };
}

/// Unwraps a DBus i64 property with custom default.
#[macro_export]
macro_rules! unwrap_i64_or {
    ($result:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            $default
        })
    };
    ($result:expr, $default:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            $default
        })
    };
}

/// Unwraps a DBus u32 property with 0 default.
#[macro_export]
macro_rules! unwrap_u32 {
    ($result:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            0u32
        })
    };
    ($result:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            0u32
        })
    };
}

/// Unwraps a DBus u32 property with custom default.
#[macro_export]
macro_rules! unwrap_u32_or {
    ($result:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            $default
        })
    };
    ($result:expr, $path:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            $default
        })
    };
}

/// Unwraps a DBus i32 property with 0 default.
#[macro_export]
macro_rules! unwrap_i32 {
    ($result:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            0i32
        })
    };
    ($result:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            0i32
        })
    };
}

/// Unwraps a DBus i32 property with custom default.
#[macro_export]
macro_rules! unwrap_i32_or {
    ($result:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            $default
        })
    };
    ($result:expr, $path:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            $default
        })
    };
}

/// Unwraps a DBus vec property with empty vec default.
#[macro_export]
macro_rules! unwrap_vec {
    ($result:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            vec![]
        })
    };
    ($result:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            vec![]
        })
    };
}

/// Unwraps a DBus vec property with custom default.
#[macro_export]
macro_rules! unwrap_vec_or {
    ($result:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            $default
        })
    };
    ($result:expr, $path:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            $default
        })
    };
}

/// Unwraps a DBus object path property with root path "/" default.
#[macro_export]
macro_rules! unwrap_path {
    ($result:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            ::zbus::zvariant::OwnedObjectPath::default()
        })
    };
    ($result:expr, $path:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            ::zbus::zvariant::OwnedObjectPath::default()
        })
    };
}

/// Unwraps a DBus object path property with custom default.
#[macro_export]
macro_rules! unwrap_path_or {
    ($result:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!("Failed to fetch '{}' property: {}", "property", err);
            $default
        })
    };
    ($result:expr, $path:expr, $default:expr) => {
        $result.unwrap_or_else(|err| {
            ::tracing::warn!(
                "Failed to fetch '{}' property for {:?}: {}",
                "property",
                $path,
                err
            );
            $default
        })
    };
}
