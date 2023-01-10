use std::ffi::CStr;
use cmd_lib::run_cmd;
use libloading::{Symbol, Library};

fn main() {
    run_cmd!(
        cd ../clap-plugin;
        cargo build;
    ).expect("Unable to run command");

    unsafe {
        let lib = Library::new("../clap-plugin/target/debug/libclap_plugin.so").expect("Library not found");
        let load_plugin: Symbol<unsafe extern fn() -> *const i8> = lib.get(b"load_plugin").expect("Symbol not found");
        let msg = load_plugin();
        // Safety: load_plugin always returns a null terminated string
        let msg = CStr::from_ptr(msg);
        println!("Status: {}.", msg.to_string_lossy());
    };
}
