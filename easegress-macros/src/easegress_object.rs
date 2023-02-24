// Copyright (c) 2017, MegaEase All rights reserved. Licensed under the Apache License, Version 2.0 (the "License");

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Error, ImplItem, Item};

pub fn expand_macro(tokens: TokenStream) -> syn::Result<TokenStream> {
    let item = syn::parse2::<Item>(tokens)?;
    match item {
        Item::Impl(imp) => {
            let impl_token = imp.impl_token;
            let trai = imp.trait_.clone();
            let (_, trai, _) =
                trai.ok_or_else(|| Error::new_spanned(impl_token, "Must be a Program trait impl"))?;
            if !trai
                .segments
                .last()
                .map(|x| x.ident == "Program")
                .unwrap_or(false)
            {
                return Err(Error::new(trai.span(), "Must be a Program trait impl"));
            }

            let pound = syn::Token![#](imp.span()).to_token_stream();

            let struct_name = imp.self_ty;
            let items = imp.items;
            let mut tokenized = vec![];

            for item in items {
                let impl_method = match item {
                    ImplItem::Method(m) => m,
                    _ => {
                        return Err(Error::new_spanned(
                            item,
                            "Impl block must only contain methods",
                        ))
                    }
                };

                let tokens = match impl_method.sig.ident.to_string().as_str() {
                    "new" => {
                        let method = impl_method.clone();
                        quote! {
                            #method
                        }
                    }
                    "run" => {
                        let method = impl_method.clone();
                        quote! {
                            #method
                        }
                    }
                    _ => panic!(),
                };
                tokenized.push(tokens);
            }

            Ok(quote! {
                impl #struct_name {
                    #(#tokenized)*
                }

                #pound[no_mangle]
                pub extern "C" fn wasm_init(ptr: i32) {
                    let mut params = HashMap::<String, String>::new();
                    let p = ptr as *const i32;
                    let length = unsafe { std::ptr::read(p) };
                    let mut data:Vec<String> = Vec::with_capacity(length as usize);
                    let mut offset = ptr + 4;
                    if length > 0 {
                        for i in 0..length {
                            let len = unsafe { std::ptr::read(offset as *const i32)};
                            offset += 4;
                            let slice = unsafe {
                                std::slice::from_raw_parts(offset as *const u8, (len - 1) as usize)
                            };
                            offset += len;
                            data.push(String::from_utf8_lossy(slice).to_string());
                        }
                        for i in (0..data.len()).step_by(2) {
                            params.insert(data.get(i).unwrap().to_owned(), data.get(i + 1).unwrap().to_owned());
                        }
                    }
                    PROGRAM.with(|t| {
                        *t.borrow_mut() = #struct_name::new(params);
                    });
                }

                #pound[no_mangle]
                pub extern "C" fn wasm_run() -> i32 {
                    PROGRAM.with(|t| {
                        t.borrow().run()
                    })
                }
            })
        }
        Item::Struct(struc) => {
            let tokens = struc.to_token_stream();
            let struct_name = struc.ident;
            Ok(quote!(
                #tokens

                thread_local!(static PROGRAM: std::cell::RefCell<#struct_name> = std::cell::RefCell::new(#struct_name::new(HashMap::<String, String>::new())));
            ))
        }
        _ => Err(Error::new(
            item.span(),
            "Easegress Object macro can only be applied to structs and their impl of Program trait",
        )),
    }
}
