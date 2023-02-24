// Copyright (c) 2017, MegaEase All rights reserved. Licensed under the Apache License, Version 2.0 (the "License");

use std::collections::HashMap;

use crate::marshal::marshal_string;

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod cluster;
pub mod cookie;
mod marshal;
pub mod request;
pub mod response;

/// wasm_alloc is an export function for Easegress. Do not use it.
#[no_mangle]
pub extern "C" fn wasm_alloc(size: i32) -> i32 {
    let buf: Vec<u8> = Vec::with_capacity(size as usize);
    buf.as_ptr() as i32
}

/// wasm_free is an export function for Easegress. Do not use it.
#[no_mangle]
pub extern "C" fn wasm_free(ptr: i32) {
    let p = ptr as *const i32;
    let length = unsafe { std::ptr::read(p) } as usize;
    let data = unsafe { Vec::from_raw_parts(ptr as *mut u8, length + 4, length + 4) };
    drop(data);
}

/// Extend the ability of Easegress by implement `Program` trait.
pub trait Program {
    /// Easegress will call `new` when initializing the WasmHost filter. You can initialize your struct here.
    ///
    /// The parameter is a `HashMap<String, String>` representing `parameters` field in the spec.
    ///
    /// e.g. With this spec:
    ///
    /// ```yaml
    /// filters:
    /// - name: wasm
    ///   kind: WasmHost
    ///   parameters:
    ///     blockRatio: "0.4"
    ///     maxPermission: "3"
    /// ```
    ///
    /// `HashMap<String, String>` contains {"blockRation": "0.4", "maxPermission": "3"}.
    fn new(params: HashMap<String, String>) -> Self;

    /// Easegress will call `run` on each request.
    fn run(&self) -> i32 {
        0
    }
}

#[link(wasm_import_module = "easegress")]
extern "C" {
    fn host_add_tag(addr: i32);
    fn host_log(level: i32, msg: i32);
    fn host_get_unix_time_in_ms() -> i64;
    fn host_rand() -> f64;
}

/// AddTag add a tag to the Request Context.
#[no_mangle]
pub fn add_tag(tag: String) {
    let data = marshal_string(tag);
    unsafe { host_add_tag(data.as_ptr() as i32) }
}

#[derive(Copy, Clone)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

/// print log in Easegress server.
#[no_mangle]
pub fn log(level: LogLevel, msg: String) {
    let data = marshal_string(msg);
    unsafe {
        host_log(level as i32, data.as_ptr() as i32);
    }
    drop(data);
}

#[no_mangle]
pub fn get_unix_time_in_ms() -> i64 {
    unsafe { host_get_unix_time_in_ms() }
}

#[no_mangle]
pub fn rand() -> f64 {
    unsafe { host_rand() }
}
