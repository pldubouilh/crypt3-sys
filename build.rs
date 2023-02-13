extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::extra_assert;

fn main() {
    cc::Build::new()
        .file("crypt256.c")
        .compile("libcrypt256.a");

    // println!("cargo:rustc-link-lib=libcrypt256");

    println!("-l:libcrypt256.a");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
