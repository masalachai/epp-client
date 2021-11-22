//! # Macros for the epp-client Library.
//!
//! ## Description
//!
//! `epp-client` is a client library for Internet domain registration and management for domain registrars.
//! This macro crate contains a few macros to simplify serialization of generic types used in some places
//! in the `epp-client` library
//!

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

fn element_name_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut elem_name = ast.ident.to_string();
    let (impl_generics, type_generics, _) = &ast.generics.split_for_impl();

    if !ast.attrs.is_empty() {
        let attribute = &ast.attrs[0];
        match attribute.parse_meta() {
            Ok(syn::Meta::List(meta)) => {
                if !meta.nested.is_empty() {
                    elem_name = match &meta.nested[0] {
                        syn::NestedMeta::Meta(syn::Meta::NameValue(v)) => match &v.lit {
                            syn::Lit::Str(lit) => lit.value(),
                            _ => panic!("Invalid element_name attribute"),
                        },
                        _ => panic!("Invalid element_name attribute"),
                    };
                } else {
                    panic!("Invalid element_name attribute");
                }
            }
            _ => panic!("Invalid element_name attribute"),
        };
    }

    let implement = quote! {
        impl #impl_generics ElementName for #name #type_generics {
            fn element_name(&self) -> &'static str {
                #elem_name
            }
        }
    };
    implement.into()
}

#[proc_macro_derive(ElementName, attributes(element_name))]
pub fn element_name_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Error while parsing ElementName macro input");

    element_name_macro(&ast)
}

fn epp_request_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let attr = &ast.attrs[0];
    let mut response_type: Option<syn::Ident> = None;
    let (impl_generics, type_generics, _) = &ast.generics.split_for_impl();

    if attr.path.is_ident("response") {
        match attr.parse_meta() {
            Ok(syn::Meta::List(meta)) => {
                let item = &meta.nested[0];
                match item {
                    syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
                        response_type = Some(p.get_ident().unwrap().clone());
                    }
                    _ => panic!("Failed to parse args for epp_types"),
                }
            }
            _ => panic!("Failed to parse args for epp_types"),
        };
    }

    if let Some(resp) = response_type {
        let implement = quote::quote! {
            impl #impl_generics EppRequest for #name #type_generics {
                type Output = #resp;

                fn deserialize_response(&self, epp_xml: &str) -> Result<Self::Output, Box<dyn std::error::Error>> {
                    match Self::Output::deserialize(epp_xml) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(format!("epp-client: Deserialization error: {}", e).into()),
                    }
                }

                fn serialize_request(&self) -> Result<String, Box<dyn std::error::Error>> {
                    match &self.0.serialize() {
                        Ok(serialized) => Ok(serialized.to_string()),
                        Err(e) => Err(format!("epp-client: Serialization error: {}", e).into()),
                    }
                }
            }
        };
        implement.into()
    } else {
        panic!(
            "response() needs 1 argument, a response type that implements epp_client::epp::xml::EppXml"
        );
    }
}

#[proc_macro_derive(EppRequest, attributes(response))]
pub fn epp_request_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).expect("Error while parsing EppTransaction macro input");

    epp_request_macro(&ast)
}
