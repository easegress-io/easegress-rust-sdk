// Copyright (c) 2017, MegaEase All rights reserved. Licensed under the Apache License, Version 2.0 (the "License");

use crate::marshal::{marshal_data, marshal_string, unmarshal_data, unmarshal_string};

#[link(wasm_import_module = "easegress")]
extern "C" {
    fn host_cluster_get_binary(addr: i32) -> i32;
    fn host_cluster_put_binary(key_adddr: i32, val_addr: i32);
    fn host_cluster_get_string(addr: i32) -> i32;
    fn host_cluster_put_string(key_addr: i32, val_addr: i32);
    fn host_cluster_get_integer(addr: i32) -> i64;
    fn host_cluster_put_integer(addr: i32, val: i64);
    fn host_cluster_add_integer(addr: i32, val: i64) -> i64;
    fn host_cluster_get_float(addr: i32) -> f64;
    fn host_cluster_put_float(addr: i32, val: f64);
    fn host_cluster_add_float(addr: i32, val: f64) -> f64;
    fn host_cluster_count_key(addr: i32) -> i32;
}

#[no_mangle]
pub fn get_binary(key: String) -> Vec<u8> {
    let v = marshal_string(key);
    let data = unsafe { host_cluster_get_binary(v.as_ptr() as i32) };
    unmarshal_data(data)
}

#[no_mangle]
pub fn put_binary(key: String, val: Vec<u8>) {
    let ptr_key = marshal_string(key);
    let ptr_val = marshal_data(val);
    unsafe { host_cluster_put_binary(ptr_key.as_ptr() as i32, ptr_val.as_ptr() as i32) }
}

#[no_mangle]
pub fn get_string(key: String) -> String {
    let v = marshal_string(key);
    let data = unsafe { host_cluster_get_string(v.as_ptr() as i32) };
    unmarshal_string(data)
}

#[no_mangle]
pub fn put_string(key: String, val: String) {
    let ptr_key = marshal_string(key);
    let ptr_val = marshal_string(val);
    unsafe { host_cluster_put_string(ptr_key.as_ptr() as i32, ptr_val.as_ptr() as i32) }
}

#[no_mangle]
pub fn get_integer(key: String) -> i64 {
    let ptr = marshal_string(key);
    unsafe { host_cluster_get_integer(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn put_integer(key: String, val: i64) {
    let ptr = marshal_string(key);
    unsafe { host_cluster_put_integer(ptr.as_ptr() as i32, val) }
}

#[no_mangle]
pub fn add_integer(key: String, val: i64) -> i64 {
    let ptr = marshal_string(key);
    unsafe { host_cluster_add_integer(ptr.as_ptr() as i32, val) }
}

#[no_mangle]
pub fn get_float(key: String) -> f64 {
    let ptr = marshal_string(key);
    unsafe { host_cluster_get_float(ptr.as_ptr() as i32) }
}

#[no_mangle]
pub fn put_float(key: String, val: f64) {
    let ptr = marshal_string(key);
    unsafe { host_cluster_put_float(ptr.as_ptr() as i32, val) }
}

#[no_mangle]
pub fn add_float(key: String, val: f64) -> f64 {
    let ptr = marshal_string(key);
    unsafe { host_cluster_add_float(ptr.as_ptr() as i32, val) }
}

#[no_mangle]
pub fn count_key(prefix: String) -> i32 {
    let v = marshal_string(prefix);
    unsafe { host_cluster_count_key(v.as_ptr() as i32) }
}
