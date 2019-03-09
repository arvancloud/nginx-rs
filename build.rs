extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const NGINX_VERSION: &'static str = "1.15.8";

fn run_make(rule: &str, cwd: &Path) -> Option<bool> {
    let output = Command::new("make")
        .arg(rule)
        .env("OUT_DIR", env::var("OUT_DIR").unwrap())
        .env("NGINX_VERSION", env::var("NGINX_VERSION").unwrap_or(NGINX_VERSION.to_string()))
        .current_dir(cwd)
        .output()
        .ok()?;
    Some(output.status.success())
}

fn main() {
    println!("cargo:rerun-if-env-changed=NGINX_VERSION");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let cwd = env::current_dir().unwrap();
    run_make("prepare-nginx", cwd.as_path())
        .and_then(|success| if success { Some(()) } else { None })
        .expect("unable to execute `make prepare-nginx`");

    let nginx_dir_path = out_path.join("nginx");
    let nginx_dir = nginx_dir_path.to_str().unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .layout_tests(false)
        .clang_args(vec![
            format!("-I{}/src/core", nginx_dir),
            format!("-I{}/src/event", nginx_dir),
            format!("-I{}/src/event/modules", nginx_dir),
            format!("-I{}/src/os/unix", nginx_dir),
            format!("-I{}/objs", nginx_dir),
            format!("-I{}/src/http", nginx_dir),
            format!("-I{}/src/http/modules", nginx_dir),
        ]).generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write bindings");
}
