#![doc = include_str!("../README.md")]

use darling::FromField;
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, ToTokens};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use syn::spanned::Spanned;

const ATTRIBUTE_NAME: &str = "serde_nested_with";

#[derive(FromField)]
#[darling(attributes(serde_nested_with))]
struct Field {
    ty: syn::Type,
    substitute: syn::Path,
    with: Option<String>,
    serialize_with: Option<String>,
    deserialize_with: Option<String>,
}

impl Field {
    /// Returns the name of the argument that should be passed to the serde operation.
    fn serde_operation(&self) -> &str {
        match self {
            Field { with: Some(_), .. } => "with",
            Field { serialize_with: Some(_), .. } => "serialize_with",
            Field { deserialize_with: Some(_), .. } => "deserialize_with",
            _ => abort!(self.substitute.span(), "missing serde operation"),
        }
    }

    /// Returns the name of the serde operation as a syn identifier.
    fn serde_operation_ident(&self) -> syn::Ident {
        syn::Ident::new(self.serde_operation(), self.ty.span())
    }

    /// Returns the name of the generated module that will be used for (de)serialization.
    fn outer_module_base_name(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.substitute.to_token_stream().to_string().hash(&mut hasher);
        self.with.hash(&mut hasher);
        self.serialize_with.hash(&mut hasher);
        self.deserialize_with.hash(&mut hasher);
        format!("__serde_nested_with_{}", hasher.finish())
    }

    /// Returns the path to the generated module or function that will be used for
    /// (de)serialization.
    fn outer_module_name_with_op(&self) -> String {
        let base_name = self.outer_module_base_name();
        match self {
            Field { with: Some(_), .. } => base_name,
            Field { serialize_with: Some(_), .. } => base_name + "::serialize",
            Field { deserialize_with: Some(_), .. } => base_name + "::deserialize",
            _ => abort!(self.substitute.span(), "missing serde operation"),
        }
    }

    /// Returns the path to the user provided module or function that will be used for
    /// (de)serialization.
    fn inner_module_name_with_op(&self) -> &str {
        match self {
            Field { with: Some(path), .. } => path,
            Field { serialize_with: Some(path), .. } => path,
            Field { deserialize_with: Some(path), .. } => path,
            _ => abort!(self.ty.span(), "missing serde operation"),
        }
    }

    /// Extracts the generic argument that is indicated by the placeholder `_`.
    fn generic_argument(&self) -> syn::Path {
        let full_ty = self.ty.to_token_stream().to_string();
        let sub_ty = self.substitute.to_token_stream().to_string();
        let (prefix, suffix) = match sub_ty.split_once('_') {
            Some(res) => res,
            None => abort!(self.substitute.span(), "missing placeholder `_`"),
        };
        let generic = full_ty
            .strip_prefix(prefix)
            .and_then(|s| s.strip_suffix(suffix))
            .and_then(|s| syn::parse_str(s).ok());
        match generic {
            Some(generic) => generic,
            None => abort!(self.substitute.span(), "placeholder does not match the type"),
        }
    }

    /// Plugs the provided generic argument in place of the placeholder `_`.
    fn plug_generic_argument(&self, generic_argument: &str) -> syn::Path {
        let ty = self.substitute.to_token_stream().to_string();
        let ty = ty.replace('_', generic_argument);
        match syn::parse_str(&ty) {
            Ok(path) => path,
            Err(_) => abort!(self.substitute.span(), "placeholder does not match the type"),
        }
    }

    /// Plugs the provided generic argument in place of the placeholder `_` and adds turbofish
    /// syntax.
    fn plug_generic_argument_turbofish(&self, generic_argument: &str) -> syn::Path {
        let ty = self.substitute.to_token_stream().to_string();
        let ty = ty.replacen('<', ":: <", 1);
        let ty = ty.replace('_', generic_argument);
        match syn::parse_str(&ty) {
            Ok(path) => path,
            Err(_) => abort!(self.substitute.span(), "placeholder does not match the type"),
        }
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn serde_nested_with(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::ItemStruct);
    let mut modules = HashMap::new();

    for field in input.fields.iter_mut() {
        let attrs = match Field::from_field(field) {
            Ok(attrs) => attrs,
            Err(_) => continue,
        };
        for attr in field.attrs.iter_mut() {
            if let syn::Meta::List(ref mut list) = attr.meta {
                if let Some(syn::PathSegment { ident, .. }) = list.path.segments.first_mut() {
                    if ident == ATTRIBUTE_NAME {
                        *ident = syn::parse_quote!(serde);
                        let serde_operation_ident = attrs.serde_operation_ident();
                        let outer_module_name_with_op = attrs.outer_module_name_with_op();
                        list.tokens =
                            quote! { #serde_operation_ident = #outer_module_name_with_op };
                        modules.insert(attrs.outer_module_base_name(), attrs);
                        break;
                    }
                }
            }
        }
    }

    let modules = modules.into_iter().map(|(outer_module_name, attrs)| {
        let outer_module_name = syn::Ident::new(&outer_module_name, attrs.ty.span());
        let inner_module_name = &attrs.inner_module_name_with_op();
        let field_ty = &attrs.ty;
        let operation = attrs.serde_operation_ident();
        let generic_argument = attrs.generic_argument();
        let wrapper_type = attrs.plug_generic_argument("__Wrapper");
        let wrapper_type_turbofish = attrs.plug_generic_argument_turbofish("__Wrapper");

        quote! {
            mod #outer_module_name {
                use super::*;
                use serde::{Serialize as _, Deserialize as _};

                #[derive(serde::Serialize, serde::Deserialize)]
                #[serde(transparent)]
                #[repr(transparent)]
                struct __Wrapper(#[serde(#operation=#inner_module_name)] #generic_argument);

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
        #(#modules)*
        #input
    };
    output.into()
}
