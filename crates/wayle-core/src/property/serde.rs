#[cfg(feature = "schema")]
use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter};

#[cfg(feature = "schema")]
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Property;

impl<T: Clone + Send + Sync + Debug + 'static> Debug for Property<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Property")
            .field("value", &self.get())
            .finish()
    }
}

impl<T: Clone + Send + Sync + Serialize + 'static> Serialize for Property<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.get().serialize(serializer)
    }
}

impl<'de, T: Clone + Send + Sync + Deserialize<'de> + 'static> Deserialize<'de> for Property<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Property::new(value))
    }
}

#[cfg(feature = "schema")]
impl<T: Clone + Send + Sync + JsonSchema + 'static> JsonSchema for Property<T> {
    fn schema_name() -> Cow<'static, str> {
        T::schema_name()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        T::json_schema(generator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_to_inner_value() {
        let property = Property::new(42);
        let json = serde_json::to_string(&property).unwrap();

        assert_eq!(json, "42");
    }

    #[test]
    fn deserializes_from_inner_value() {
        let property: Property<String> = serde_json::from_str("\"hello\"").unwrap();

        assert_eq!(property.get(), "hello");
    }

    #[test]
    fn json_roundtrip() {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct Config {
            name: Property<String>,
            count: Property<i32>,
            enabled: Property<bool>,
        }

        let config = Config {
            name: Property::new(String::from("test")),
            count: Property::new(42),
            enabled: Property::new(true),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name.get(), "test");
        assert_eq!(deserialized.count.get(), 42);
        assert!(deserialized.enabled.get());
    }

    #[test]
    fn toml_roundtrip() {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct ClockConfig {
            format: Property<String>,
            show_seconds: Property<bool>,
        }

        let config = ClockConfig {
            format: Property::new(String::from("%H:%M")),
            show_seconds: Property::new(false),
        };

        let serialized = toml::to_string(&config).unwrap();
        assert!(serialized.contains("format = \"%H:%M\""));
        assert!(serialized.contains("show_seconds = false"));

        let deserialized: ClockConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.format.get(), "%H:%M");
        assert!(!deserialized.show_seconds.get());
    }

    #[test]
    fn deserialized_property_starts_with_no_subscribers() {
        let property: Property<i32> = serde_json::from_str("42").unwrap();

        assert!(!property.has_subscribers());
    }
}
