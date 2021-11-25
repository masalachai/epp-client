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
            const ELEMENT: &'static str = #elem_name;
        }
    };
    implement.into()
}

#[proc_macro_derive(ElementName, attributes(element_name))]
pub fn element_name_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Error while parsing ElementName macro input");

    element_name_macro(&ast)
}
