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

/// Derive macro for `UpdateFromToml` trait.
///
/// Automatically generates code to update struct fields from TOML values.
///
/// # Requirements
///
/// - Only works with structs that have named fields
/// - All fields must implement `UpdateFromToml`
/// - Field names in TOML must match struct field names exactly
///
/// # Behavior
///
/// Updates are applied field-by-field. Missing TOML fields are silently ignored,
/// allowing partial updates. Type mismatches are handled by the field's
/// `UpdateFromToml` implementation.
///
/// # Example
///
/// ```ignore
/// use wayle_common::{Property, UpdateFromToml};
/// use wayle_derive::UpdateFromToml;
///
/// #[derive(UpdateFromToml)]
/// struct BatteryConfig {
///     enabled: Property<bool>,
///     low_threshold: Property<u32>,
/// }
/// ```
#[proc_macro_derive(UpdateFromToml)]
pub fn derive_update_from_toml(input: TokenStream) -> TokenStream {
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
                self.#field_name.update_from_toml(field_value);
            }
        }
    });

    let update_fields = quote! {
        if let toml::Value::Table(table) = value {
            #(#field_updates)*
        }
    };

    let expanded = quote! {
        impl wayle_common::UpdateFromToml for #name {
            fn update_from_toml(&self, value: &toml::Value) {
                #update_fields
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
