extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // Include the library.
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let c_header_path = Path::new(&cargo_dir).join("c-src").join("lib.h");
    let c_header = c_header_path.to_str().unwrap();

    let c_build_dir_path = Path::new(&cargo_dir).join("build");
    let c_build_dir = c_build_dir_path.to_str().unwrap();

    println!("cargo:rustc-link-lib=dylib=sum_bindgen");
    println!("cargo:rustc-link-search={c_build_dir}");

    let bindings = bindgen::Builder::default()
        .header(c_header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
