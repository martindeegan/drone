extern crate protoc_rust;

use std::fs;

fn main() {


//    let CARGO_MANIFEST_DIR = concat!(env!("CARGO_MANIFEST_DIR"));
//    println!("manifest: {}", CARGO_MANIFEST_DIR);

    fs::create_dir("src/generated");

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/generated",
        includes: &["protos"],
        input: &[
            "protos/vector3.proto",
            "protos/position.proto",
            "protos/controller_input.proto"
        ]
    }).expect("protoc");
}

