extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

fn element_name_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut elem_name = ast.ident.to_string();
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    if ast.attrs.len() > 0 {
        let attribute = &ast.attrs[0];
        let meta = match attribute.parse_meta() {
            Ok(syn::Meta::List(meta)) => {
                if meta.nested.len() > 0 {
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

// #[proc_macro_attribute]
// pub fn epp_client_command_response(_metadat: TokenStream, input: TokenStream) -> TokenStream {
//     let mut ast = parse_macro_input!(input as DeriveInput);

//     match &mut ast.data {
//         syn::Data::Struct(ref mut data) => {
//             match &mut data.fields {
//                 syn::Fields::Named(fields) => {
//                     fields.named.push(
//                         syn::Field::parse_named
//                             .parse2(quote! {
//                                 pub result: EppResult
//                             })
//                             .unwrap()
//                     );
//                     fields.named.push(
//                         syn::Field::parse_named
//                             .parse2(quote! {
//                                 pub tr_ids: ResponseTRID
//                             })
//                             .unwrap()
//                     );
//                 }
//                 _ => (),
//             }

//             return quote! { #ast }.into();
//         }
//         _ => panic!("Failed to parse CommandResponse macro input"),
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
