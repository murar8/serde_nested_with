#![doc = include_str!("../README.md")]

use darling::FromField;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{quote, ToTokens};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash as _, Hasher};
use syn::spanned::Spanned;

#[derive(Debug, Clone, FromField)]
#[darling(attributes(serde_nested))]
struct Field {
    ty: syn::Type,
    #[darling(rename = "sub")]
    substitute: syn::Path,
    #[darling(rename = "serde")]
    serde_attr: syn::Meta,
    #[darling(multiple, rename = "derive_trait")]
    derive_traits: Vec<syn::Path>,
}

impl Field {
    /// Generate a unique name for the module that will contain the wrapper type and the
    /// (de)serialization functions. The name is based on the field type, the substitute type,
    /// and the serde attributes. In this way, we do not generate duplicate modules.
    fn module_name(&self, struct_name: &syn::Ident) -> String {
        let hasher = &mut DefaultHasher::new();
        struct_name.hash(hasher);
        self.ty.hash(hasher);
        self.substitute.hash(hasher);
        self.serde_attr.hash(hasher);
        format!("__serde_nested_{}", hasher.finish())
    }

    /// Get the ident of the wrapper type that will be used to (de)serialize the field.
    ///
    /// Example: `Option<OffsetDateTime>` -> `Option<__Wrapper>`
    fn wrapper_type_ident(&self) -> syn::Path {
        let ty = self.ty.to_token_stream().to_string();
        let sub = self.substitute.to_token_stream().to_string();
        let ty = ty.replace(&sub, "__Wrapper");
        syn::parse_str(&ty).unwrap()
    }

    /// Get the ident of the wrapper type that will be used to (de)serialize the field, with
    /// turbofish syntax.
    ///
    /// Example: `Option<OffsetDateTime>` -> `Option::<__Wrapper>`
    fn wrapper_type_turbofish(&self) -> syn::Path {
        let ty = self.wrapper_type_ident().to_token_stream().to_string();
        let ty = ty.replacen('<', " :: <", 1);
        syn::parse_str(&ty).unwrap()
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn serde_nested(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::ItemStruct);

    // Contains the field information for each module.
    let mut fields = HashMap::new();

    for field in input.fields.iter_mut() {
        let info = match Field::from_field(field) {
            Ok(field) => field,
            Err(_) => continue,
        };

        // Remove all #[serde(...)] attributes from the field so we can merge them with our own.
        let mut serde_attributes = Vec::new();
        field.attrs.retain(|attr| {
            if let syn::Meta::List(ref list) = attr.meta {
                if let Some(syn::PathSegment { ident, .. }) = list.path.segments.first() {
                    if ident == "serde" {
                        serde_attributes.push(list.tokens.clone());
                        return false;
                    }
                }
            }
            true
        });

        // Replace #[serde_nested] with #[serde(with = "module_name", ...)] and store the field
        // information for later use.
        for attr in field.attrs.iter_mut() {
            if let syn::Meta::List(ref mut list) = attr.meta {
                if let Some(syn::PathSegment { ident, .. }) = list.path.segments.first_mut() {
                    if ident == "serde_nested" {
                        *ident = syn::parse_quote!(serde);
                        let module_name = info.module_name(&input.ident);
                        list.tokens = quote! { with = #module_name, #(#serde_attributes),* };
                        fields.insert(module_name, info.clone());
                    }
                }
            }
        }
    }

    let modules = fields.into_iter().map(|(module_name, field)| {
        let module_name = syn::Ident::new(&module_name, input.span());
        let serde_attr = &field.serde_attr;
        let substitute = &field.substitute;
        let inner_ty = &field.ty;
        let wrapper_type = field.wrapper_type_ident();
        let wrapper_type_turbofish = field.wrapper_type_turbofish();
        let derive_traits = field.derive_traits;

        quote! {
            mod #module_name {
                use super::*;
                use serde::{Serialize as _, Deserialize as _};

                #[derive(serde::Serialize, serde::Deserialize, #(#derive_traits),*)]
                #[serde(transparent)]
                #[repr(transparent)]
                struct __Wrapper(#[#serde_attr] #substitute);

                pub fn serialize<S: serde::Serializer>(
                    val: &#inner_ty,
                    serializer: S
                ) -> std::result::Result<S::Ok, S::Error> {
                    // SAFETY: __Wrapper is #[repr(transparent)] and has the same size and alignment
                    // as the field type.
                    let val: &#wrapper_type = unsafe { std::mem::transmute(val) };
                    val.serialize(serializer)
                }

                pub fn deserialize<'de, D>(
                    deserializer: D
                ) -> std::result::Result<#inner_ty, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let v = #wrapper_type_turbofish::deserialize(deserializer)?;
                    // SAFETY: Same as in serialize.
                    Ok(unsafe { std::mem::transmute(v) })
                }
            }
        }
    });

    let output = quote! {
        #( #modules )*
        #input
    };

    output.into()
}
