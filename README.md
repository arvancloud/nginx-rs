# nginx-rs

[![crates.io](https://img.shields.io/crates/v/nginx.svg)](https://crates.io/crates/nginx) [![Documentation](https://img.shields.io/badge/Docs-nginx-blue.svg)](https://arvancloud.github.io/nginx-rs) [![Build Status](https://travis-ci.org/arvancloud/nginx-rs.svg?branch=master)](https://travis-ci.org/arvancloud/nginx-rs) ![Crates.io](https://img.shields.io/crates/l/rustc-serialize.svg) ![Nginx](https://img.shields.io/badge/nginx-1.19.0-orange.svg)

This crate provides [nginx](https://nginx.org/) bindings for Rust. Currently, only Linux is supported.

## How to Use

1. Add `nginx` crate to Cargo.toml

```toml
[dependencies]
nginx = "0.9"
```

**Note:** In order to build the crate, `clang` must be installed.

## Environment Variables

- `NGINX_VERSION` Determines the version of nginx, if it is not set, the default version is used.
- `NGINX_PATH` Determines the local absolute path of pre-cloned nginx, if it is not set, nginx is downloaded.

Some code were copied (and refactored) from [nginxinc/ngx-rust](https://github.com/nginxinc/ngx-rust).
