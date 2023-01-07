use bindgen;
use cc;
use std::path::Path;

fn main() {

    //let cur_dir = std::env::current_dir().unwrap();

    cc::Build
        ::new()        
        .include(Path::new("C:/projects/RUST/clap-wrap/clap-main/include/clap"))
        .includes(Path::new("C:/projects/RUST/clap-wrap/clap-main/src"))
        .compile("clapc");

    let bindings = 
        bindgen::Builder
            ::default()
            .header("wrapper.h")
            .generate()
            .expect("Failed to generate bindings");

    //let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings   
        .write_to_file(Path::new("C:/projects/RUST/clap-wrap/src/bindings.rs"))
        .expect("Failed to write bindings");
        

}
