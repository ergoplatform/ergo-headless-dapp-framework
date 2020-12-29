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

#[proc_macro_derive(WASMBox)]
pub fn wasm_box_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_wasm_box(&ast)
}

fn impl_wasm_box(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[wasm_bindgen]
        impl #name {
            #[wasm_bindgen(constructor)]
            pub fn w_new(ergo_box: WErgoBox) -> std::result::Result<#name, JsValue> {
                let b: ErgoBox = ergo_box.into();
                Self::box_spec()
                    .verify_box(&b)
                    .map_err(|e| JsValue::from_str(&format! {"{:?}", e}))?;
                Ok(#name {
                    ergo_box: b.clone(),
                })
            }

            #[wasm_bindgen]
            pub fn w_box_spec(&self) -> BoxSpec {
                Self::box_spec()
            }

            #[wasm_bindgen]
            pub fn w_process_explorer_response(explorer_response_body: &str)
                -> std::result::Result<Vec<JsValue>, JsValue> {
                let boxes = Self::process_explorer_response(explorer_response_body)
                                .map_err(|err| JsValue::from_str(&format!("{}", err)))?;
                Ok(boxes.into_iter().map(JsValue::from).collect())
            }

            #[wasm_bindgen]
            pub fn w_explorer_endpoint(explorer_api_url: &str) -> std::result::Result<String, JsValue> {
                Self::box_spec().explorer_endpoint(explorer_api_url)
                                .map_err(|err| JsValue::from_str(&format!("{}", err)))
            }
        }

    };
    gen.into()
}
