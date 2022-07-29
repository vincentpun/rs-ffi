extern crate cbindgen;

use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_dir_path = Path::new(&crate_dir).join("c-src");
    let output_file_path = output_dir_path.join("main.h");

    cbindgen::Builder::new()
        .with_language(cbindgen::Language::C)
        .with_crate(crate_dir)
        .generate()
        .unwrap()
        .write_to_file(output_file_path.to_str().unwrap());
}
