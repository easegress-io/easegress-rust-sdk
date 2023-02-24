// Copyright (c) 2017, MegaEase All rights reserved. Licensed under the Apache License, Version 2.0 (the "License");

use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct Cookie {
    name: String,
    value: String,
    path: String,
    domain: String,
    raw_expires: String,
    max_age: i32,
    secure: bool,
    http_only: bool,
    same_site: SameSite,
}

#[derive(Debug, Clone)]
pub enum SameSite {
    DefaultMode = 0,
    LaxMode = 1,
    StrictMode = 2,
    NoneMode = 3,
}

impl Cookie {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_value(&self) -> &str {
        self.value.as_str()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn get_domain(&self) -> &str {
        self.domain.as_str()
    }

    pub fn set_domain(&mut self, domain: String) {
        self.domain = domain;
    }

    pub fn get_raw_expires(&self) -> &str {
        self.raw_expires.as_str()
    }

    pub fn set_raw_expires(&mut self, raw_expires: String) {
        self.raw_expires = raw_expires;
    }

    pub fn get_max_age(&self) -> i32 {
        self.max_age
    }

    pub fn set_max_age(&mut self, age: i32) {
        self.max_age = age;
    }

    pub fn get_secure(&self) -> bool {
        self.secure
    }

    pub fn set_secure(&mut self, secure: bool) {
        self.secure = secure;
    }

    pub fn get_http_only(&self) -> bool {
        self.http_only
    }

    pub fn set_http_only(&mut self, val: bool) {
        self.http_only = val;
    }

    pub fn get_same_site(&self) -> &SameSite {
        self.same_site.borrow()
    }

    pub fn set_same_site(&mut self, val: SameSite) {
        self.same_site = val;
    }

    pub fn marshal(&self) -> String {
        assert!(self.name.len() > 0, "cookie name must be specified");
        let mut str = "".to_string();
        str += format!("{}={}", self.name, self.value).as_str();

        if self.path.len() > 0 {
            str += format!("; Path={}", self.path).as_str();
        }

        if self.domain.len() > 0 {
            str += format!("; Domain={}", self.domain).as_str();
        }

        if self.raw_expires.len() > 0 {
            str += format!("; Expires={}", self.raw_expires).as_str();
        }

        if self.max_age > 0 {
            str += format!("; Max-Age={}", self.raw_expires).as_str();
        }

        if self.secure {
            str += "; Secure";
        }

        if self.http_only {
            str += "; HttpOnly";
        }

        match self.same_site {
            SameSite::DefaultMode => {}
            SameSite::LaxMode => str += "; SameSite=Lax",
            SameSite::StrictMode => str += "; SameSite=Strict",
            SameSite::NoneMode => str += "; SameSite=None",
        }

        return str;
    }

    pub fn unmarshal(str: String) -> Option<Cookie> {
        let parts: Vec<_> = str.split(";").collect();
        let kv: Vec<_> = parts[0].trim().split("=").collect();
        if kv.len() != 2 {
            return None;
        }
        let mut c = Cookie::default();
        c.set_name(kv[0].to_string());
        c.set_value(kv[1].to_string());

        for part in parts {
            let kv: Vec<_> = part.trim().split("=").collect();
            if kv.len() != 2 {
                continue;
            }

            let k = kv[0].to_lowercase();
            if k == "path" {
                c.set_path(kv[1].to_string());
            } else if k == "domain" {
                c.set_domain(kv[1].to_string());
            } else if k == "expires" {
                c.set_raw_expires(kv[1].to_string());
            } else if k == "max-age" {
                c.set_max_age(kv[1].to_string().parse::<i32>().unwrap());
            } else if k == "secure" {
                c.set_secure(true);
            } else if k == "httponly" {
                c.set_http_only(true);
            } else if k == "samesite" {
                let v = kv[1].to_lowercase();
                if v == "lax" {
                    c.set_same_site(SameSite::LaxMode);
                } else if v == "strict" {
                    c.set_same_site(SameSite::StrictMode);
                } else if v == "none" {
                    c.set_same_site(SameSite::NoneMode);
                } else {
                    c.set_same_site(SameSite::DefaultMode);
                }
            }
        }

        return Some(c);
    }
}

impl Default for Cookie {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            value: "".to_string(),
            path: "".to_string(),
            domain: "".to_string(),
            raw_expires: "".to_string(),
            max_age: 0,
            secure: false,
            http_only: false,
            same_site: SameSite::DefaultMode,
        }
    }
}
