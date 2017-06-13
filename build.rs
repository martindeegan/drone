#![allow(dead_code)]
extern crate protoc_rust;

use std::env;
use std::fs;

fn main() {
    fs::create_dir("src/proto");

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/proto",
        input: &["proto/position.proto"],
        includes: &["proto"],
    }).expect("protoc");

    match env::var("$RUST_PI_COMPILATION") {
        Ok(val) => println!("cargo:rustc-cfg={:?}", val),
        Err(e) => println!("cargo:warning=WARNING: RASPBERRY PI NOT DETECTED. SKIPPING RPI ONLY MODULES. {:?}", e) //Pi not detected
    }
}