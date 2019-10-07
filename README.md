# nginx-rs

[![crates.io](https://img.shields.io/crates/v/nginx.svg)](https://crates.io/crates/nginx) [![Documentation](https://img.shields.io/badge/Docs-nginx-blue.svg)](https://docs.rs/nginx) [![Build Status](https://travis-ci.org/arvancloud/nginx-rs.svg?branch=master)](https://travis-ci.org/arvancloud/nginx-rs) ![Crates.io](https://img.shields.io/crates/l/rustc-serialize.svg) ![Nginx](https://img.shields.io/badge/Nginx-1.17.4-orange.svg)

This crate provides [nginx](https://nginx.org/) bindings for Rust. Currently, only Linux is supported.

## How to Use

1. Add `nginx` crate to Cargo.toml

```toml
[dependencies]
nginx = "0.5"
```

**Note:** In order to build the crate, `clang` must be installed and the following command must be executed on the host:

```sh
sed -i 's:# define IPPORT_RESERVED:// #define IPPORT_RESERVED:' /usr/include/netdb.h
```

## Build

It is recommended to use [Docker](https://docs.docker.com/) to build the crate:

```sh
make build-image
make build
```

## Environment Variables

- `NGINX_VERSION` Determines the version of nginx, if it is not set, the default version is used.
- `NGINX_PATH` Determines the local absolute path of pre-cloned nginx, if it is not set, nginx is downloaded.

Some code were copied (and refactored) from [nginxinc/ngx-rust](https://github.com/nginxinc/ngx-rust).
