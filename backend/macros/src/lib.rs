extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, parse_str, Ident, ItemFn, Stmt};

fn handle_types(input_fn: ItemFn) -> Stmt {
    let statements: Vec<Stmt> = input_fn
        .sig
        .inputs
        .iter()
        .skip(1)
        .map(|input| {
            if let syn::FnArg::Typed(arg) = input {
                let arg_name = &arg.pat;

                match arg.ty.to_token_stream().to_string().as_str() {
                    "String" => {
                        parse_quote! {
                            let #arg_name = values.remove(0);
                        }
                    }
                    "& str" => {
                        parse_quote! {
                            let #arg_name = values.remove(0).as_str();
                        }
                    }
                    type_str => {
                        let ty: Ident = parse_str(&type_str).unwrap();
                        parse_quote! {
                            let #arg_name = values.remove(0).parse::<#ty>()?;
                        }
                    }
                }
            } else {
                parse_quote! {}
            }
        })
        .collect();

    let old_body = input_fn.clone().block;
    let new_body: Stmt = parse_quote! {
        {
            #(#statements)*
            #old_body
        }
    };

    new_body
}

fn get_name(input_fn: &ItemFn) -> Ident {
    let fn_name = &input_fn.sig.ident;
    Ident::new(&fn_name.to_string()[2..], fn_name.span())
}

fn get_arg_name(input_fn: &ItemFn) -> Box<syn::Pat> {
    match input_fn.sig.inputs.iter().next() {
        Some(syn::FnArg::Typed(arg)) => arg.pat.clone(),
        _ => panic!("No arguments found"),
    }
}

#[proc_macro_attribute]
pub fn getter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_clone = item.clone();
    let input_fn = parse_macro_input!(item_clone as ItemFn);   
    let new_body = handle_types(input_fn.clone());

    let fn_new_name = get_name(&input_fn);
    let arg_name = get_arg_name(&input_fn);

    let gen = quote! {
        #input_fn

        pub fn #fn_new_name(#arg_name: &mut Context, mut values: Vec<String>) -> anyhow::Result<String> {
            #new_body
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_clone = item.clone();
    let input_fn = parse_macro_input!(item_clone as ItemFn);   
    let new_body = handle_types(input_fn.clone());

    let fn_new_name = get_name(&input_fn);
    let arg_name = get_arg_name(&input_fn);

    let gen = quote! {
        #input_fn

        pub fn #fn_new_name(#arg_name: &mut Context, mut values: Vec<String>) -> anyhow::Result<()> {
            #new_body
        }
    };

    gen.into()
}
