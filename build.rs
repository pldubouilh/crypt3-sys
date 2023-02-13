extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let files = [
        "crypt_musl/crypt_blowfish.c",
        "crypt_musl/crypt.c",
        "crypt_musl/crypt_des.c",
        "crypt_musl/crypt_md5.c",
        "crypt_musl/crypt_sha256.c",
        "crypt_musl/crypt_sha512.c",
        "crypt_musl/encrypt.c",
    ];

    cc::Build::new()
        .cargo_metadata(true)
        .warnings(false)
        .files(files)
        .file("crypt_musl/crypt.c")
        .compile("libcrypt.a");

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
