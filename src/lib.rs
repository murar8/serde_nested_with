#![doc = include_str!("../README.md")]

use darling::FromField;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::{quote, ToTokens};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash as _, Hasher};
use syn::spanned::Spanned;

const WRAPPER_NAME: &str = "__Wrapper";
const ATTRIBUTE_NAME: &str = "serde_nested";

#[derive(Debug, FromField)]
#[darling(attributes(serde_nested))]
struct Field {
    ty: syn::Type,
    sub: syn::Path,
    serde: syn::Meta,
}

impl Field {
    fn module_name(&self) -> String {
        let hasher = &mut DefaultHasher::new();
        self.ty.hash(hasher);
        self.sub.hash(hasher);
        self.serde.hash(hasher);
        format!("__serde_nested_{}", hasher.finish())
    }

    fn wrapper_type(&self) -> String {
        let ty = self.ty.to_token_stream().to_string();
        let sub = self.sub.to_token_stream().to_string();
        ty.replace(&sub, WRAPPER_NAME)
    }

    fn wrapper_type_ident(&self) -> syn::Path {
        let ty = self.wrapper_type();
        syn::parse_str(&ty).unwrap()
    }

    fn wrapper_type_turbofish(&self) -> syn::Path {
        let ty = self.wrapper_type();
        let ty = ty.replacen('<', " :: <", 1);
        syn::parse_str(&ty).unwrap()
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn serde_nested(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::ItemStruct);
    let mut fields = HashMap::new();

    for field in input.fields.iter_mut() {
        let info = match Field::from_field(field) {
            Ok(field) => field,
            Err(_) => continue,
        };
        for attr in field.attrs.iter_mut() {
            if let syn::Meta::List(ref mut list) = attr.meta {
                if let Some(syn::PathSegment { ident, .. }) = list.path.segments.first_mut() {
                    if ident == ATTRIBUTE_NAME {
                        *ident = syn::parse_quote!(serde);
                        let module_name = info.module_name();
                        list.tokens = quote! { with = #module_name };
                        fields.insert(module_name, info);
                        break;
                    }
                }
            }
        }
    }

    let modules = fields.into_iter().map(|(module_name, field)| {
        let module_name = syn::Ident::new(&module_name, input.span());
        let field_serde_attr = &field.serde;
        let field_sub = &field.sub;
        let field_ty = &field.ty;
        let wrapper_type = field.wrapper_type_ident();
        let wrapper_type_turbofish = field.wrapper_type_turbofish();

        quote! {
            mod #module_name {
                use super::*;
                use serde::{Serialize as _, Deserialize as _};

                #[derive(serde::Serialize, serde::Deserialize)]
                #[serde(transparent)]
                #[repr(transparent)]
                struct __Wrapper(#[#field_serde_attr] #field_sub);

                pub fn serialize<S: serde::Serializer>(
                    val: &#field_ty,
                    serializer: S
                ) -> std::result::Result<S::Ok, S::Error> {
                    // SAFETY: __Wrapper is #[repr(transparent)] and has the same size and alignment
                    // as the field type.
                    let val: &#wrapper_type = unsafe { std::mem::transmute(val) };
                    val.serialize(serializer)
                }

                pub fn deserialize<'de, D>(
                    deserializer: D
                ) -> std::result::Result<#field_ty, D::Error>
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
