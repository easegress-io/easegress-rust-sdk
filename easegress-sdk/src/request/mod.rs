// Copyright (c) 2017, MegaEase All rights reserved. Licensed under the Apache License, Version 2.0 (the "License");

use std::collections::HashMap;

use crate::cookie::Cookie;
use crate::marshal::{
    marshal_all_header, marshal_cookie, marshal_data, marshal_string, unmarshal_all_header,
    unmarshal_cookie, unmarshal_data, unmarshal_string, unmarshal_string_vec,
};

#[link(wasm_import_module = "easegress")]
extern "C" {
    fn host_req_get_real_ip() -> i32;
    fn host_req_get_scheme() -> i32;
    fn host_req_get_proto() -> i32;
    fn host_req_get_method() -> i32;
    fn host_req_set_method(addr: i32);
    fn host_req_get_host() -> i32;
    fn host_req_set_host(addr: i32);
    fn host_req_get_path() -> i32;
    fn host_req_set_path(addr: i32);
    fn host_req_get_escaped_path() -> i32;
    fn host_req_get_query() -> i32;
    fn host_req_set_query(addr: i32);
    fn host_req_get_fragment() -> i32;
    fn host_req_get_header(addr: i32) -> i32;
    fn host_req_get_all_header() -> i32;
    fn host_req_set_header(name_addr: i32, value_addr: i32);
    fn host_req_set_all_header(addr: i32);
    fn host_req_add_header(name_addr: i32, value_addr: i32);
    fn host_req_del_header(addr: i32);
    fn host_req_get_cookie(addr: i32) -> i32;
    fn host_req_get_all_cookie() -> i32;
    fn host_req_add_cookie(addr: i32);
    fn host_req_get_body() -> i32;
    fn host_req_set_body(addr: i32);
}

#[no_mangle]
pub fn get_real_ip() -> String {
    let ptr = unsafe { host_req_get_real_ip() };
    unmarshal_string(ptr)
}

#[no_mangle]
pub fn get_scheme() -> String {
    let ptr = unsafe { host_req_get_scheme() };
    unmarshal_string(ptr)
}

#[no_mangle]
pub fn get_proto() -> String {
    let ptr = unsafe { host_req_get_proto() };
    unmarshal_string(ptr)
}

#[no_mangle]
pub fn get_method() -> String {
    let ptr = unsafe { host_req_get_method() };
    unmarshal_string(ptr)
}

#[no_mangle]
pub fn set_method(method: String) {
    let ptr = marshal_string(method);
    unsafe { host_req_set_method(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn get_host() -> String {
    let ptr = unsafe { host_req_get_host() };
    unmarshal_string(ptr)
}

#[no_mangle]
pub fn set_host(host: String) {
    let ptr = marshal_string(host);
    unsafe { host_req_set_host(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn get_path() -> String {
    let ptr = unsafe { host_req_get_path() };
    unmarshal_string(ptr)
}

#[no_mangle]
pub fn set_path(path: String) {
    let ptr = marshal_string(path);
    unsafe { host_req_set_path(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn get_escape_path() -> String {
    let ptr = unsafe { host_req_get_escaped_path() };
    unmarshal_string(ptr)
}

#[no_mangle]
pub fn get_query() -> String {
    let ptr = unsafe { host_req_get_query() };
    unmarshal_string(ptr)
}

#[no_mangle]
pub fn set_query(query: String) {
    let ptr = marshal_string(query);
    unsafe { host_req_set_query(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn get_fragment() -> String {
    let ptr = unsafe { host_req_get_fragment() };
    unmarshal_string(ptr)
}

#[no_mangle]
pub fn get_header(name: String) -> String {
    let ptr = marshal_string(name);
    let ptr = unsafe { host_req_get_header(ptr.as_ptr() as i32) };
    drop(ptr);
    unmarshal_string(ptr)
}

#[no_mangle]
pub fn get_all_header() -> HashMap<String, Vec<String>> {
    let ptr = unsafe { host_req_get_all_header() };
    unmarshal_all_header(ptr)
}

#[no_mangle]
pub fn set_header(name: String, value: String) {
    let name_ptr = marshal_string(name);
    let value_ptr = marshal_string(value);
    unsafe { host_req_set_header(name_ptr.as_ptr() as i32, value_ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn set_all_header(headers: HashMap<String, Vec<String>>) {
    let ptr = marshal_all_header(headers);
    unsafe { host_req_set_all_header(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn add_header(name: String, value: String) {
    let name_ptr = marshal_string(name);
    let value_ptr = marshal_string(value);
    unsafe { host_req_add_header(name_ptr.as_ptr() as i32, value_ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn del_header(name: String) {
    let ptr = marshal_string(name);
    unsafe { host_req_del_header(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn get_cookie(name: String) -> Option<Cookie> {
    let ptr = marshal_string(name);
    let data = unsafe { host_req_get_cookie(ptr.as_ptr() as i32) };
    unmarshal_cookie(data)
}

#[no_mangle]
pub fn get_all_cookie() -> Vec<Cookie> {
    let mut result = Vec::<Cookie>::new();
    let ptr = unsafe { host_req_get_all_cookie() };
    let strs = unmarshal_string_vec(ptr);
    for str in strs {
        let c = Cookie::unmarshal(str);
        if c.is_some() {
            result.push(c.unwrap());
        }
    }
    result
}

#[no_mangle]
pub fn add_cookie(c: Cookie) {
    let ptr = marshal_cookie(c);
    unsafe { host_req_add_cookie(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn get_body() -> Vec<u8> {
    let ptr = unsafe { host_req_get_body() };
    unmarshal_data(ptr)
}

#[no_mangle]
pub fn set_body(body: Vec<u8>) {
    let ptr = marshal_data(body);
    unsafe { host_req_set_body(ptr.as_ptr() as i32) }
}
