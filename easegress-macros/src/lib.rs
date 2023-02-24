// Copyright (c) 2017, MegaEase All rights reserved. Licensed under the Apache License, Version 2.0 (the "License");

//！#[easegress_object] is an annotation to generate the `wasm_init` and `wasm_run` functions.
//! #[easegress_object] can only be used on your struct and the implementation of the Program trait for your struct.
//!
//! # Examples
//! ```
//! #[easegress_object]
//! struct Fake;
//!
//! #[easegress_object]
//! impl Program for Fake {
//!     fn new(_param: std::collections::HashMap<String, String>) -> Self {
//!         Self {}
//!     }
//!
//!     fn run(&self) -> i32 {
//!         0
//!     }
//! }
//! ```
//!
//! # Errors
//! ```
//! #[easegress_object]
//! fn fake() {}
//! ```

mod easegress_object;

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn easegress_object(_attr: TokenStream, item: TokenStream) -> TokenStream {
    easegress_object::expand_macro(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
