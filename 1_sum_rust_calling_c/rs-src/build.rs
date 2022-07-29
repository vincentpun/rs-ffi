use std::env;
use std::path::Path;

fn main() {
    // Include the library.
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let c_build_dir_path = Path::new(&cargo_dir).join("build");
    let c_build_dir = c_build_dir_path.to_str().unwrap();
    println!("cargo:rustc-link-lib=dylib=sum_rust_calling_c");
    println!("cargo:rustc-link-search={c_build_dir}");
}
