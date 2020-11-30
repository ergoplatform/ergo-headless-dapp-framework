extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(WrapBox)]
pub fn wrapped_box_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_wrapped_box(&ast)
}

fn impl_wrapped_box(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl WrappedBox for #name {
            fn get_box(&self) -> ErgoBox {
                self.ergo_box.clone()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SpecBox)]
pub fn specified_box_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_specified_box(&ast)
}

fn impl_specified_box(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub fn new(b: &ErgoBox) -> std::result::Result<#name, HeadlessDappError> {
                Self::box_spec().verify_box(&b)?;
                return Ok(#name {
                    ergo_box: b.clone(),
                });
            }
        }

        impl ExplorerFindable for #name {
            fn process_explorer_response(explorer_response_body: &str) -> std::result::Result<Vec<#name>, HeadlessDappError> {
                Self::process_explorer_response_custom(explorer_response_body, #name::box_spec())
            }

            fn process_explorer_response_custom(explorer_response_body: &str, box_spec: BoxSpec) -> std::result::Result<Vec<#name>, HeadlessDappError> {
                let boxes = box_spec.process_explorer_response(explorer_response_body)?;
                let mut specified_boxes = vec![];
                for b in boxes {
                    specified_boxes.push(Self::new(&b)?);
                }
                Ok(specified_boxes)
            }
        }

    };
    gen.into()
}
