use std::path::PathBuf;
use bindgen;

fn main() {
    let bindings =
        bindgen::Builder
            ::default()
            .header("wrapper.h")
            .generate()
            .expect("Failed to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
