#!/bin/bash

cd /nginx-rs
RUSTFLAGS=-Awarnings cargo build -j`nproc`
