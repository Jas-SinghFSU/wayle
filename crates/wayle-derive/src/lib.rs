//! Derive macros for Wayle configuration management.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed, parse_macro_input};

fn validate_named_struct(input: &DeriveInput) -> Result<&FieldsNamed, TokenStream> {
    match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => Ok(fields),
            _ => Err(syn::Error::new_spanned(
                input,
                "Can only be derived for structs with named fields",
            )
            .to_compile_error()
            .into()),
        },
        _ => Err(syn::Error::new_spanned(
            input,
            "Can only be derived for structs",
        )
        .to_compile_error()
        .into()),
    }
}

/// Derive macro for `ApplyConfigLayer` trait.
///
/// Walks struct fields and applies TOML values to their config layer.
/// Used when loading config.toml.
///
/// # Generated Code
///
/// For each field, generates: `self.field.apply_config_layer(&toml["field"])`
#[proc_macro_derive(ApplyConfigLayer)]
pub fn derive_apply_config_layer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let field_updates = fields.named.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            if let Some(field_value) = table.get(stringify!(#field_name)) {
                self.#field_name.apply_config_layer(field_value);
            }
        }
    });

    let expanded = quote! {
        impl wayle_common::ApplyConfigLayer for #name {
            fn apply_config_layer(&self, value: &toml::Value) {
                if let toml::Value::Table(table) = value {
                    #(#field_updates)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for `ApplyRuntimeLayer` trait.
///
/// Walks struct fields and applies TOML values to their runtime layer.
/// Used when loading runtime.toml (GUI overrides).
///
/// # Generated Code
///
/// For each field, generates: `self.field.apply_runtime_layer(&toml["field"])`
#[proc_macro_derive(ApplyRuntimeLayer)]
pub fn derive_apply_runtime_layer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let field_updates = fields.named.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            if let Some(field_value) = table.get(stringify!(#field_name)) {
                self.#field_name.apply_runtime_layer(field_value);
            }
        }
    });

    let expanded = quote! {
        impl wayle_common::ApplyRuntimeLayer for #name {
            fn apply_runtime_layer(&self, value: &toml::Value) {
                if let toml::Value::Table(table) = value {
                    #(#field_updates)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for `ExtractRuntimeValues` trait.
///
/// Walks struct fields and collects runtime layer values into a TOML table.
/// Used when persisting runtime.toml.
///
/// # Generated Code
///
/// For each field with a runtime value, adds it to the output table.
/// Returns None if no fields have runtime values.
#[proc_macro_derive(ExtractRuntimeValues)]
pub fn derive_extract_runtime_values(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let field_extractions = fields.named.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            if let Some(value) = self.#field_name.extract_runtime_values() {
                table.insert(String::from(stringify!(#field_name)), value);
            }
        }
    });

    let expanded = quote! {
        impl wayle_common::ExtractRuntimeValues for #name {
            fn extract_runtime_values(&self) -> Option<toml::Value> {
                let mut table = toml::map::Map::new();
                #(#field_extractions)*
                if table.is_empty() {
                    None
                } else {
                    Some(toml::Value::Table(table))
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for `SubscribeChanges` trait.
///
/// Automatically generates code to subscribe to changes in all struct fields.
///
/// # Requirements
///
/// - Only works with structs that have named fields
/// - All fields must implement `SubscribeChanges`
///
/// # Behavior
///
/// Subscribes to each field's changes by calling `subscribe_changes` with a cloned
/// channel sender. When any field changes, a notification is sent to the channel.
///
/// # Example
///
/// ```ignore
/// use wayle_common::{Property, SubscribeChanges};
/// use wayle_derive::SubscribeChanges;
///
/// #[derive(SubscribeChanges)]
/// struct BatteryConfig {
///     enabled: Property<bool>,
///     low_threshold: Property<u32>,
/// }
/// ```
#[proc_macro_derive(SubscribeChanges)]
pub fn derive_subscribe_changes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let field_subscriptions = fields.named.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            self.#field_name.subscribe_changes(tx.clone());
        }
    });

    let subscribe_fields = quote! {
        #(#field_subscriptions)*
    };

    let expanded = quote! {
        impl wayle_common::SubscribeChanges for #name {
            fn subscribe_changes(&self, tx: tokio::sync::mpsc::UnboundedSender<()>) {
                #subscribe_fields
            }
        }
    };

    TokenStream::from(expanded)
}
