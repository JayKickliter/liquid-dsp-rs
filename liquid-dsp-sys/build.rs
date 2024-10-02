use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-lib=liquid");

    let bindings = bindgen::Builder::default()
        .clang_arg("-I/opt/homebrew/include")
        .clang_arg("-DLIQUID_SUPPRESS_ERROR_OUTPUT")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
