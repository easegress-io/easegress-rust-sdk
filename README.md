# Easegress Rust SDK
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Feasegress-io%2Feasegress-rust-sdk.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Feasegress-io%2Feasegress-rust-sdk?ref=badge_shield)


- [Easegress Rust SDK](#easegress-rust-sdk)
	- [Prerequisites](#prerequisites)
	- [Local Development](#local-development)
	- [Deploy and execute](#deploy-and-execute)

This is the [Rust](https://www.rust-lang.org/) SDK for [Easegress](https://github.com/megaease/easegress). It can be used to extend the ability of Easegress.

## Prerequisites

The following steps assume that [Git](https://git-scm.com/) and [Rust](https://www.rust-lang.org/) are installed.

## Local Development

1. Clone the repo.

```bash
git clone https://github.com/megaease/easegress-rust-demo.git
```

2. Implement your extension in `src/lib.rs`. Please check the `examples` directory for more examples.

```rust
use std::collections::HashMap;
use std::time::Duration;
use easegress_sdk::*;
use easegress_macros::easegress_object;

// define your own struct.
#[easegress_object]
struct AddRequestHeader;

// implement Program trait for your own struct.
#[easegress_object]
impl Program for AddRequestHeader {
    fn new(_params: HashMap<String, String>) -> Self {
        Self {}
    }

    fn run(&self) -> i32 {
        let v = request::get_header("Foo".to_string());
        if v.len() > 10 {
            log(LogLevel::Warning, format!("The length of Foo is greater than 10"));
        }

        if v.len() > 0 {
            request::add_header("Wasm-Added".to_string(), v.clone());
        }

        request::set_body("I have a new body now".as_bytes().to_vec());
        0
    }
}
```

**Note**

* You need to implement the `Program` trait on your own struct. 
* Additionally, the `#[easegress_object]` attribute macro must be applied to both your struct definition and the trait impl block for it.
* Only one struct with `#[easegress_object]` attribute macro is allowed.


3. Add `wasm32-unknown-unknown` target.

```bash
rustup target add wasm32-unknown-unknown 
```

4. Build with this command

```bash
cargo build --target wasm32-unknown-unknown --release
```

If success, it will generate `easegress_demo.wasm` at the `target/wasm32-unknown-unknown/release` folder.

## Deploy and execute

Please refer to [the documentation of `WasmHost`](https://github.com/megaease/easegress/blob/main/doc/reference/wasmhost.md) for deploying and executing the compiled Wasm code.


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Feasegress-io%2Feasegress-rust-sdk.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Feasegress-io%2Feasegress-rust-sdk?ref=badge_large)