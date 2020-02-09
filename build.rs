extern crate bindgen;

use std::env;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

const NGINX_VERSION: &'static str = "1.17.8";

fn run_make(rule: &str, cwd: &Path, local_nginx_path: &str) -> Result<bool> {
    let output = Command::new("make")
        .arg(rule)
        .env("OUT_DIR", env::var("OUT_DIR").unwrap())
        .env(
            "NGINX_VERSION",
            env::var("NGINX_VERSION").unwrap_or(NGINX_VERSION.to_string()),
        )
        .env("NGINX_PATH", local_nginx_path)
        .current_dir(cwd)
        .output()?;
    Ok(output.status.success())
}

fn main() {
    println!("cargo:rerun-if-env-changed=NGINX_VERSION");
    println!("cargo:rerun-if-env-changed=NGINX_PATH");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let cwd = env::current_dir().unwrap();
    let local_nginx_path = env::var("NGINX_PATH").unwrap_or_default();

    run_make(
        if local_nginx_path.is_empty() {
            "prepare-nginx"
        } else {
            "prepare-nginx-local"
        },
        cwd.as_path(),
        local_nginx_path.as_str(),
    )
    .map_err(|e| e.to_string())
    .and_then(|success| {
        if success {
            Ok(())
        } else {
            Err(String::from(
                "preparing nginx exited with non-zero status code",
            ))
        }
    })
    .expect("unable to prepare nginx");

    let nginx_dir_path = out_path.join("nginx");
    let nginx_dir = if local_nginx_path.is_empty() {
        nginx_dir_path.to_str().unwrap()
    } else {
        local_nginx_path.as_str()
    };

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .layout_tests(false)
        .blacklist_item("IPPORT_RESERVED")
        .clang_args(vec![
            format!("-I{}/src/core", nginx_dir),
            format!("-I{}/src/event", nginx_dir),
            format!("-I{}/src/event/modules", nginx_dir),
            format!("-I{}/src/os/unix", nginx_dir),
            format!("-I{}/objs", nginx_dir),
            format!("-I{}/src/http", nginx_dir),
            format!("-I{}/src/http/v2", nginx_dir),
            format!("-I{}/src/http/modules", nginx_dir),
        ])
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write bindings");
}
