#![allow(missing_docs)]

use wayle_common::{ConfigProperty, SubscribeChanges};
use wayle_derive::SubscribeChanges;

#[derive(SubscribeChanges)]
struct SimpleConfig {
    enabled: ConfigProperty<bool>,
    count: ConfigProperty<u32>,
}

#[tokio::test]
async fn derives_subscribe_changes_for_simple_struct() {
    let config = SimpleConfig {
        enabled: ConfigProperty::new(false),
        count: ConfigProperty::new(0),
    };

    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    config.subscribe_changes(tx);
}

#[derive(SubscribeChanges)]
struct NestedConfig {
    simple: SimpleConfig,
    name: ConfigProperty<String>,
}

#[tokio::test]
async fn derives_subscribe_changes_for_nested_struct() {
    let config = NestedConfig {
        simple: SimpleConfig {
            enabled: ConfigProperty::new(false),
            count: ConfigProperty::new(0),
        },
        name: ConfigProperty::new("old".to_string()),
    };

    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    config.subscribe_changes(tx);
}
