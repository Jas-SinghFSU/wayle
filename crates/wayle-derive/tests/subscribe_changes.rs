#![allow(missing_docs)]

use wayle_common::{Property, SubscribeChanges};
use wayle_derive::SubscribeChanges;

#[derive(SubscribeChanges)]
struct SimpleConfig {
    enabled: Property<bool>,
    count: Property<u32>,
}

#[tokio::test]
async fn derives_subscribe_changes_for_simple_struct() {
    let config = SimpleConfig {
        enabled: Property::new(false),
        count: Property::new(0),
    };

    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    config.subscribe_changes(tx);
}

#[derive(SubscribeChanges)]
struct NestedConfig {
    simple: SimpleConfig,
    name: Property<String>,
}

#[tokio::test]
async fn derives_subscribe_changes_for_nested_struct() {
    let config = NestedConfig {
        simple: SimpleConfig {
            enabled: Property::new(false),
            count: Property::new(0),
        },
        name: Property::new("old".to_string()),
    };

    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    config.subscribe_changes(tx);
}
