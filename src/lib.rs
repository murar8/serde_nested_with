use darling::FromField;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

const ATTRIBUTE_NAME: &str = "serde_nested_with";

#[derive(Debug, Clone, Hash, FromField)]
#[darling(attributes(serde_nested_with))]
struct Field {
    ty: syn::Type,
    substitute: syn::Path,
    with: String,
}

impl Field {
    fn module_name(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.substitute.hash(&mut hasher);
        format!("__serde_nested_with_{}", hasher.finish())
    }

    fn get_generic_argument(&self) -> syn::Path {
        let full_ty = self.ty.to_token_stream().to_string();
        let sub_ty = self.substitute.to_token_stream().to_string();
        let placeholder_idx = sub_ty.find('_').expect("placeholder not found");
        let (prefix, suffix) = sub_ty.split_at(placeholder_idx);
        let suffix = &suffix[1..];
        let generic = full_ty.strip_prefix(prefix).unwrap().strip_suffix(suffix).unwrap();
        syn::parse_str::<syn::Path>(generic).expect("failed to parse generic argument")
    }

    fn plug_generic_argument(&self, generic_argument: &str) -> syn::Path {
        let ty = self.substitute.to_token_stream().to_string();
        let ty = ty.replace('_', generic_argument);
        syn::parse_str::<syn::Path>(&ty).expect("failed to parse generic argument")
    }

    fn plug_generic_argument_with_path(&self, generic_argument: &str) -> syn::Path {
        let ty = self.substitute.to_token_stream().to_string();
        let ty = ty.replacen('<', ":: <", 1);
        let ty = ty.replace('_', generic_argument);
        syn::parse_str::<syn::Path>(&ty).expect("failed to parse generic argument")
    }
}

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
                        let module_name = attrs.module_name();
                        list.tokens = quote! { with = #module_name };
                        modules.insert(module_name, attrs);
                        break;
                    }
                }
            }
        }
    }

    let modules = modules.into_iter().map(|(module_name, attrs)| {
        let module_name_id = syn::Ident::new(&module_name, proc_macro::Span::call_site().into());
        let inner_module_name = &attrs.with;
        let field_ty = &attrs.ty;
        let generic_argument = attrs.get_generic_argument();
        let wrapper_type = attrs.plug_generic_argument("Wrapper");
        let wrapper_type_with_path = attrs.plug_generic_argument_with_path("Wrapper");

        quote! {
            mod #module_name_id {
                use super::*;
                use serde::{Serialize, Deserialize};

                #[derive(serde::Serialize, serde::Deserialize)]
                #[serde(transparent)]
                struct Wrapper(#[serde(with=#inner_module_name)] #generic_argument);

                pub fn serialize<S: serde::Serializer>(
                    val: &#field_ty,
                    serializer: S
                ) -> std::result::Result<S::Ok, S::Error> {
                    let val: &#wrapper_type = unsafe { std::mem::transmute(val) };
                    val.serialize(serializer)
                }

                pub fn deserialize<'de, D>(
                    deserializer: D
                ) -> std::result::Result<#field_ty, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let v = #wrapper_type_with_path::deserialize(deserializer)?;
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
