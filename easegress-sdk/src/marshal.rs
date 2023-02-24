// Copyright (c) 2017, MegaEase All rights reserved. Licensed under the Apache License, Version 2.0 (the "License");

use crate::cookie::Cookie;
use std::collections::HashMap;

/// marshal Vec<u8>
/// -------------------------------
/// | vec ...
/// -------------------------------
///              to
/// -------------------------------
/// | vec len (4 bytes) | vec ...
/// -------------------------------
pub fn marshal_data(data: Vec<u8>) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::with_capacity(data.len() + 4);
    let length = data.len() as i32;
    buf.extend(length.to_le_bytes());
    buf.extend(data);
    buf
}

/// unmarshal Vec<u8>
/// -------------------------------
/// | vec len (4 bytes) | vec ...
/// -------------------------------
/// ^
/// ptr
///             to
/// -------------------------------
/// | vec ...
/// -------------------------------
pub fn unmarshal_data(ptr: i32) -> Vec<u8> {
    let p = ptr as *const i32;
    let length = unsafe { std::ptr::read(p) };
    let data =
        unsafe { std::slice::from_raw_parts((ptr + 4) as *const u8, length as usize) }.to_vec();
    data
}

/// marshal string to Vec<u8>
/// -----------------------------------------
/// | string len (4 bytes) | string ... | 0 |
/// -----------------------------------------
pub fn marshal_string(data: String) -> Vec<u8> {
    let len = data.len() as i32 + 1;
    let mut buf: Vec<u8> = Vec::with_capacity(data.len() + 5);
    buf.extend(len.to_le_bytes());
    buf.extend(data.as_bytes());
    buf.push(0);
    buf
}

/// unmarshal string from ptr
/// -----------------------------------------
/// | string len (4 bytes) | string ... | 0 |
/// -----------------------------------------
/// ^
/// ptr
/// -------------------------------------
/// | string ...
/// -------------------------------------
pub fn unmarshal_string(ptr: i32) -> String {
    let p = ptr as *const i32;
    let length = unsafe { std::ptr::read(p) };
    let data = unsafe { std::slice::from_raw_parts((ptr + 4) as *const u8, (length - 1) as usize) };
    String::from_utf8_lossy(data).to_string()
}

/// unmarshal string vec
/// --------------------------------------------------------------------------------------
/// | vec len (4 bytes) | string len (4 bytes) | string ... | 0 | string len (4 bytes) ...
/// --------------------------------------------------------------------------------------
pub fn unmarshal_string_vec(ptr: i32) -> Vec<String> {
    let p = ptr as *const i32;
    let length = unsafe { std::ptr::read(p) };
    let mut data = Vec::with_capacity(length as usize);
    let mut offset = ptr + 4;
    for _ in 0..length {
        let len = unsafe { std::ptr::read(offset as *const i32) };
        offset += 4;
        let slice = unsafe { std::slice::from_raw_parts(offset as *const u8, (len - 1) as usize) };
        offset += len;
        data.push(String::from_utf8_lossy(slice).to_string());
    }
    data
}

pub fn marshal_all_header(headers: HashMap<String, Vec<String>>) -> Vec<u8> {
    let mut str = "".to_string();
    for (key, val) in headers.iter() {
        for v in val.iter() {
            str += format!("{}:{}\r\n", *key, *v).as_str();
        }
    }
    return marshal_string(str);
}

pub fn unmarshal_all_header(ptr: i32) -> HashMap<String, Vec<String>> {
    let str = unmarshal_string(ptr);
    let headers: Vec<_> = str.split("\r\n").collect();
    let mut result = HashMap::<String, Vec<String>>::new();

    for header in headers {
        let kv: Vec<_> = header.split(":").collect();
        if kv.len() != 2 {
            continue;
        }
        if result.contains_key(kv[0]) {
            let v = result.get_mut(kv[0]).unwrap();
            v.push(kv[1].to_string().to_owned());
        } else {
            let mut v = Vec::<String>::new();
            v.push(kv[1].to_string().to_owned());
            result.insert(kv[0].to_string().to_owned(), v.to_owned());
        }
    }
    result
}

pub fn marshal_cookie(c: Cookie) -> Vec<u8> {
    let str = c.marshal();
    marshal_string(str)
}

pub fn unmarshal_cookie(ptr: i32) -> Option<Cookie> {
    let str = unmarshal_string(ptr);
    if str == "" {
        return None;
    }
    Cookie::unmarshal(str)
}
