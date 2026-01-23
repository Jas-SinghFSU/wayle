//! Derive macros for Wayle configuration management.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Expr, Field, Fields, FieldsNamed, Ident, ItemStruct,
    parse_macro_input,
};

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
        _ => Err(
            syn::Error::new_spanned(input, "Can only be derived for structs")
                .to_compile_error()
                .into(),
        ),
    }
}

fn should_skip(field: &Field) -> bool {
    field.attrs.iter().any(|attr| {
        if !attr.path().is_ident("wayle") {
            return false;
        }

        let mut skip = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                skip = true;
            }
            Ok(())
        });
        skip
    })
}

fn serde_key(field: &Field) -> String {
    for attr in &field.attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let mut rename = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let value: syn::LitStr = meta.value()?.parse()?;
                rename = Some(value.value());
            }
            Ok(())
        });

        if let Some(name) = rename {
            return name;
        }
    }

    field
        .ident
        .as_ref()
        .map(|i| i.to_string())
        .unwrap_or_default()
}

/// Derive macro for `ApplyConfigLayer` trait.
///
/// Walks struct fields and applies TOML values to their config layer.
/// Used when loading config.toml.
///
/// # Attributes
///
/// - `#[wayle(skip)]` - Skip this field in config layer application
///
/// # Generated Code
///
/// For each field, generates: `self.field.apply_config_layer(&toml["field"], "path.field")`
#[proc_macro_derive(ApplyConfigLayer, attributes(wayle))]
pub fn derive_apply_config_layer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let field_updates = fields
        .named
        .iter()
        .filter(|field| !should_skip(field))
        .map(|field| {
            let field_name = &field.ident;
            let key = serde_key(field);

            quote! {
                if let Some(field_value) = table.get(#key) {
                    let child_path = if path.is_empty() {
                        String::from(#key)
                    } else {
                        format!("{}.{}", path, #key)
                    };
                    self.#field_name.apply_config_layer(field_value, &child_path);
                }
            }
        });

    let expanded = quote! {
        impl wayle_common::ApplyConfigLayer for #name {
            fn apply_config_layer(&self, value: &toml::Value, path: &str) {
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
/// # Attributes
///
/// - `#[wayle(skip)]` - Skip this field in runtime layer application
///
/// # Generated Code
///
/// For each field, generates: `self.field.apply_runtime_layer(&toml["field"], "path.field")?`
#[proc_macro_derive(ApplyRuntimeLayer, attributes(wayle))]
pub fn derive_apply_runtime_layer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let field_updates = fields
        .named
        .iter()
        .filter(|field| !should_skip(field))
        .map(|field| {
            let field_name = &field.ident;
            let key = serde_key(field);

            quote! {
                if let Some(field_value) = table.get(#key) {
                    let child_path = if path.is_empty() {
                        String::from(#key)
                    } else {
                        format!("{}.{}", path, #key)
                    };
                    self.#field_name.apply_runtime_layer(field_value, &child_path)?;
                }
            }
        });

    let expanded = quote! {
        impl wayle_common::ApplyRuntimeLayer for #name {
            fn apply_runtime_layer(&self, value: &toml::Value, path: &str) -> Result<(), String> {
                if let toml::Value::Table(table) = value {
                    #(#field_updates)*
                }
                Ok(())
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
/// # Attributes
///
/// - `#[wayle(skip)]` - Skip this field in runtime value extraction
///
/// # Generated Code
///
/// For each field with a runtime value, adds it to the output table.
/// Returns None if no fields have runtime values.
#[proc_macro_derive(ExtractRuntimeValues, attributes(wayle))]
pub fn derive_extract_runtime_values(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let field_extractions = fields
        .named
        .iter()
        .filter(|field| !should_skip(field))
        .map(|field| {
            let field_name = &field.ident;
            let key = serde_key(field);

            quote! {
                if let Some(value) = self.#field_name.extract_runtime_values() {
                    table.insert(String::from(#key), value);
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

/// Derive macro for `ResetConfigLayer` trait.
///
/// Recursively clears the config layer of all fields without notifying watchers.
/// Part of the reload cycle: reset -> apply -> commit.
///
/// # Attributes
///
/// - `#[wayle(skip)]` - Skip this field in config reset
///
/// # Generated Code
///
/// For each field, generates: `self.field.reset_config_layer()`
#[proc_macro_derive(ResetConfigLayer, attributes(wayle))]
pub fn derive_reset_config_layer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let field_resets = fields
        .named
        .iter()
        .filter(|field| !should_skip(field))
        .map(|field| {
            let field_name = &field.ident;
            quote! {
                self.#field_name.reset_config_layer();
            }
        });

    let expanded = quote! {
        impl wayle_common::ResetConfigLayer for #name {
            fn reset_config_layer(&self) {
                #(#field_resets)*
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for `ResetRuntimeLayer` trait.
///
/// Recursively clears the runtime layer of all fields without notifying watchers.
/// Part of the runtime reload cycle: reset -> apply -> commit.
///
/// # Attributes
///
/// - `#[wayle(skip)]` - Skip this field in runtime reset
///
/// # Generated Code
///
/// For each field, generates: `self.field.reset_runtime_layer()`
#[proc_macro_derive(ResetRuntimeLayer, attributes(wayle))]
pub fn derive_reset_runtime_layer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let field_resets = fields
        .named
        .iter()
        .filter(|field| !should_skip(field))
        .map(|field| {
            let field_name = &field.ident;
            quote! {
                self.#field_name.reset_runtime_layer();
            }
        });

    let expanded = quote! {
        impl wayle_common::ResetRuntimeLayer for #name {
            fn reset_runtime_layer(&self) {
                #(#field_resets)*
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for `ClearRuntimeByPath` trait.
///
/// Navigates to a nested field by path and clears its runtime value.
/// Used by CLI reset commands for string-based path access.
///
/// # Attributes
///
/// - `#[wayle(skip)]` - Skip this field in path navigation
///
/// # Generated Code
///
/// Generates a match on the first path segment, delegating to child fields.
#[proc_macro_derive(ClearRuntimeByPath, attributes(wayle))]
pub fn derive_clear_runtime_by_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let match_arms = fields
        .named
        .iter()
        .filter(|field| !should_skip(field))
        .map(|field| {
            let field_name = &field.ident;
            let key = serde_key(field);

            quote! {
                #key => self.#field_name.clear_runtime_by_path(rest),
            }
        });

    let expanded = quote! {
        impl wayle_common::ClearRuntimeByPath for #name {
            fn clear_runtime_by_path(&self, path: &str) -> Result<bool, String> {
                let (segment, rest) = match path.split_once('.') {
                    Some((seg, rest)) => (seg, rest),
                    None => (path, ""),
                };

                match segment {
                    #(#match_arms)*
                    "" => Err(String::from("empty path")),
                    other => Err(format!("unknown field '{other}'")),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for `CommitConfigReload` trait.
///
/// Recursively commits config reload by recomputing effective values.
/// Part of the reload cycle: reset -> apply -> commit.
///
/// # Attributes
///
/// - `#[wayle(skip)]` - Skip this field in config commit
///
/// # Generated Code
///
/// For each field, generates: `self.field.commit_config_reload()`
#[proc_macro_derive(CommitConfigReload, attributes(wayle))]
pub fn derive_commit_config_reload(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let field_commits = fields
        .named
        .iter()
        .filter(|field| !should_skip(field))
        .map(|field| {
            let field_name = &field.ident;
            quote! {
                self.#field_name.commit_config_reload();
            }
        });

    let expanded = quote! {
        impl wayle_common::CommitConfigReload for #name {
            fn commit_config_reload(&self) {
                #(#field_commits)*
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for `SubscribeChanges` trait.
///
/// Automatically generates code to subscribe to changes in all struct fields.
///
/// # Attributes
///
/// - `#[wayle(skip)]` - Skip this field in change subscription
///
/// # Requirements
///
/// - Only works with structs that have named fields
/// - All non-skipped fields must implement `SubscribeChanges`
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
///     #[wayle(skip)]
///     runtime_only: Property<String>,
/// }
/// ```
#[proc_macro_derive(SubscribeChanges, attributes(wayle))]
pub fn derive_subscribe_changes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match validate_named_struct(&input) {
        Ok(fields) => fields,
        Err(err) => return err,
    };

    let field_subscriptions =
        fields
            .named
            .iter()
            .filter(|field| !should_skip(field))
            .map(|field| {
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

/// Attribute macro for Wayle configuration structs.
///
/// Reduces boilerplate by automatically adding all required derives and generating
/// the `Default` implementation from inline `#[default(...)]` attributes.
///
/// # Variants
///
/// - `#[wayle_config]` - Standard config struct
/// - `#[wayle_config(bar_button)]` - Bar module config with injected standard fields
///
/// # Generates
///
/// - Standard derives: `Debug`, `Clone`, `Serialize`, `Deserialize`, `JsonSchema`
/// - Config layer derives: `ApplyConfigLayer`, `ApplyRuntimeLayer`, `ExtractRuntimeValues`,
///   `SubscribeChanges`, `ResetConfigLayer`, `CommitConfigReload`
/// - `#[serde(default)]` attribute
/// - `impl Default` from `#[default(...)]` field attributes
///
/// # Field Attributes
///
/// - `#[default(expr)]` - Leaf field with `ConfigProperty<T>`, uses `ConfigProperty::new(expr)`
/// - No `#[default]` - Container field, uses `FieldType::default()`
/// - `#[serde(...)]` - Preserved and passed through to the struct
///
/// # Bar Button Fields (required by `bar_button`)
///
/// When using `#[wayle_config(bar_button)]`, these fields must be defined:
///
/// | Field | Type | Default | TOML Name |
/// |-------|------|---------|-----------|
/// | `border_show` | `bool` | `false` | `border-show` |
/// | `border_color` | `ColorValue` | `Token(BorderAccent)` | `border-color` |
/// | `icon_show` | `bool` | `true` | `icon-show` |
/// | `icon_color` | `ColorValue` | `Auto` | `icon-color` |
/// | `icon_bg_color` | `ColorValue` | `Token(Accent)` | `icon-bg-color` |
/// | `label_show` | `bool` | `true` | `label-show` |
/// | `label_color` | `ColorValue` | `Token(Accent)` | `label-color` |
/// | `label_max_length` | `Option<u32>` | `None` | `label-max-length` |
/// | `button_bg_color` | `ColorValue` | `Token(BgSurfaceElevated)` | `button-bg-color` |
/// | `left_click` | `String` | `""` | `left-click` |
/// | `right_click` | `String` | `""` | `right-click` |
/// | `middle_click` | `String` | `""` | `middle-click` |
/// | `scroll_up` | `String` | `""` | `scroll-up` |
/// | `scroll_down` | `String` | `""` | `scroll-down` |
///
/// # Example
///
/// ```ignore
/// use wayle_common::ConfigProperty;
/// use wayle_derive::wayle_config;
///
/// // Standard config
/// #[wayle_config]
/// pub struct ClockConfig {
///     #[default("%H:%M")]
///     pub format: ConfigProperty<String>,
/// }
///
/// // Bar button config with injected fields
/// #[wayle_config(bar_button)]
/// pub struct BatteryConfig {
///     // Module-specific fields only - bar button fields are injected
///     #[serde(rename = "level-icons")]
///     #[default(vec![String::from("tb-battery")])]
///     pub level_icons: ConfigProperty<Vec<String>>,
/// }
/// ```
#[proc_macro_attribute]
pub fn wayle_config(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let is_bar_button = parse_bar_button_attr(attr);

    match generate_wayle_config(input, is_bar_button) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn parse_bar_button_attr(attr: TokenStream) -> bool {
    if attr.is_empty() {
        return false;
    }

    let attr2: TokenStream2 = attr.into();
    let ident: Result<Ident, _> = syn::parse2(attr2);

    matches!(ident, Ok(id) if id == "bar_button")
}

fn generate_wayle_config(input: ItemStruct, is_bar_button: bool) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let vis = &input.vis;
    let generics = &input.generics;
    let attrs = filter_non_default_attrs(&input.attrs);

    let fields = match &input.fields {
        Fields::Named(fields) => fields,
        _ => {
            return Err(syn::Error::new_spanned(
                &input,
                "wayle_config only supports structs with named fields",
            ));
        }
    };

    if is_bar_button {
        validate_bar_button_fields(fields)?;
    }

    let processed_fields = process_fields(fields)?;
    let struct_fields = &processed_fields.struct_fields;
    let default_initializers = &processed_fields.default_initializers;

    Ok(quote! {
        #(#attrs)*
        #[derive(
            Debug,
            Clone,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema,
            wayle_derive::ApplyConfigLayer,
            wayle_derive::ApplyRuntimeLayer,
            wayle_derive::ExtractRuntimeValues,
            wayle_derive::SubscribeChanges,
            wayle_derive::ResetConfigLayer,
            wayle_derive::ResetRuntimeLayer,
            wayle_derive::ClearRuntimeByPath,
            wayle_derive::CommitConfigReload,
        )]
        #[serde(default)]
        #vis struct #name #generics {
            #(#struct_fields),*
        }

        impl #generics Default for #name #generics {
            fn default() -> Self {
                Self {
                    #(#default_initializers),*
                }
            }
        }
    })
}

const BAR_BUTTON_REQUIRED_FIELDS: &[&str] = &[
    "border_show",
    "border_color",
    "icon_show",
    "icon_color",
    "icon_bg_color",
    "label_show",
    "label_color",
    "label_max_length",
    "button_bg_color",
    "left_click",
    "right_click",
    "middle_click",
    "scroll_up",
    "scroll_down",
];

fn validate_bar_button_fields(fields: &FieldsNamed) -> syn::Result<()> {
    let field_names: Vec<String> = fields
        .named
        .iter()
        .filter_map(|f| f.ident.as_ref().map(|i| i.to_string()))
        .collect();

    let missing: Vec<&str> = BAR_BUTTON_REQUIRED_FIELDS
        .iter()
        .filter(|required| !field_names.contains(&(**required).to_string()))
        .copied()
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(syn::Error::new_spanned(
            fields,
            format!(
                "bar_button config missing required fields: {}",
                missing.join(", ")
            ),
        ))
    }
}

struct ProcessedFields {
    struct_fields: Vec<TokenStream2>,
    default_initializers: Vec<TokenStream2>,
}

fn process_fields(fields: &FieldsNamed) -> syn::Result<ProcessedFields> {
    let mut struct_fields = Vec::new();
    let mut default_initializers = Vec::new();

    for field in &fields.named {
        let field_name = field
            .ident
            .as_ref()
            .ok_or_else(|| syn::Error::new_spanned(field, "expected named field"))?;
        let field_ty = &field.ty;
        let field_vis = &field.vis;

        let (default_expr, remaining_attrs) = extract_default_attr(&field.attrs)?;

        let struct_field = quote! {
            #(#remaining_attrs)*
            #field_vis #field_name: #field_ty
        };
        struct_fields.push(struct_field);

        let initializer = match default_expr {
            Some(expr) => quote! { #field_name: wayle_common::ConfigProperty::new(#expr) },
            None => quote! { #field_name: Default::default() },
        };
        default_initializers.push(initializer);
    }

    Ok(ProcessedFields {
        struct_fields,
        default_initializers,
    })
}

fn extract_default_attr(attrs: &[Attribute]) -> syn::Result<(Option<Expr>, Vec<&Attribute>)> {
    let mut default_expr = None;
    let mut remaining = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("default") {
            if default_expr.is_some() {
                return Err(syn::Error::new_spanned(
                    attr,
                    "duplicate #[default] attribute",
                ));
            }
            default_expr = Some(attr.parse_args::<Expr>()?);
        } else {
            remaining.push(attr);
        }
    }

    Ok((default_expr, remaining))
}

fn filter_non_default_attrs(attrs: &[Attribute]) -> Vec<&Attribute> {
    attrs
        .iter()
        .filter(|attr| !attr.path().is_ident("default"))
        .collect()
}
