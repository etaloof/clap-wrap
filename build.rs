use bindgen;
use cc;

fn main() {

    cc::Build
        ::new()
        .include("clap-main")
        .compile("clapc");
  
    let bindings = 
        bindgen::Builder
            ::default()
            .header("wrapper.h")
            .generate()
            .expect("Failed to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings   
        .write_to_file("src/bindings.rs")
        .expect("Failed to write bindings");

}