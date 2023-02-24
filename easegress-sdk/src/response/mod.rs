// Copyright (c) 2017, MegaEase All rights reserved. Licensed under the Apache License, Version 2.0 (the "License");

use crate::cookie::Cookie;
use crate::marshal::{
    marshal_all_header, marshal_cookie, marshal_data, marshal_string, unmarshal_all_header,
    unmarshal_data, unmarshal_string,
};
use std::collections::HashMap;

#[link(wasm_import_module = "easegress")]
extern "C" {
    fn host_resp_get_status_code() -> i32;
    fn host_resp_set_status_code(code: i32);
    fn host_resp_get_header(addr: i32) -> i32;
    fn host_resp_get_all_header() -> i32;
    fn host_resp_set_header(name_addr: i32, value_addr: i32);
    fn host_resp_set_all_header(addr: i32);
    fn host_resp_add_header(name_addr: i32, value_addr: i32);
    fn host_resp_del_header(addr: i32);
    fn host_resp_set_cookie(addr: i32);
    fn host_resp_get_body() -> i32;
    fn host_resp_set_body(addr: i32);
}

#[no_mangle]
pub fn get_status_code() -> i32 {
    unsafe { host_resp_get_status_code() }
}

#[no_mangle]
pub fn set_status_code(code: i32) {
    unsafe { host_resp_set_status_code(code) }
}

#[no_mangle]
pub fn resp_get_header(name: String) -> String {
    let ptr = marshal_string(name);
    let data = unsafe { host_resp_get_header(ptr.as_ptr() as i32) };
    unmarshal_string(data)
}

#[no_mangle]
pub fn resp_get_all_header() -> HashMap<String, Vec<String>> {
    let ptr = unsafe { host_resp_get_all_header() };
    unmarshal_all_header(ptr)
}

#[no_mangle]
pub fn resp_set_header(name: String, value: String) {
    let name_ptr = marshal_string(name);
    let value_ptr = marshal_string(value);
    unsafe { host_resp_set_header(name_ptr.as_ptr() as i32, value_ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn resp_set_all_header(headers: HashMap<String, Vec<String>>) {
    let ptr = marshal_all_header(headers);
    unsafe { host_resp_set_all_header(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn resp_add_header(name: String, value: String) {
    let name_ptr = marshal_string(name);
    let value_ptr = marshal_string(value);
    unsafe { host_resp_add_header(name_ptr.as_ptr() as i32, value_ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn resp_del_header(name: String) {
    let ptr = marshal_string(name);
    unsafe { host_resp_del_header(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn resp_set_cookie(c: Cookie) {
    let ptr = marshal_cookie(c);
    unsafe { host_resp_set_cookie(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn resp_get_body() -> Vec<u8> {
    let ptr = unsafe { host_resp_get_body() };
    unmarshal_data(ptr)
}

#[no_mangle]
pub fn resp_set_body(body: Vec<u8>) {
    let ptr = marshal_data(body);
    unsafe { host_resp_set_body(ptr.as_ptr() as i32) }
}
