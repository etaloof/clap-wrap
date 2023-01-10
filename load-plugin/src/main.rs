use clap_wrap::{clap_plugin_entry_t, clap_plugin_factory_t};
use cmd_lib::run_cmd;
use libloading::{Library, Symbol};
use std::ops::Deref;

fn main() {
    // compile plugin crate
    run_cmd!(
        cd ../clap-plugin;
        cargo build;
    )
    .expect("Unable to run command");

    unsafe {
        // load dynamic plugin library
        let lib = Library::new("../clap-plugin/target/debug/libclap_plugin.so")
            .expect("Library not found");
        let clap_entry: Symbol<*const clap_plugin_entry_t> =
            lib.get(b"clap_entry").expect("Symbol not found");
        let clap_entry = clap_entry.deref();
        println!("plugin entry struct is at {:?}", clap_entry);
        let clap_entry = clap_entry.as_ref().unwrap();

        // check version
        assert!(matches!(
            clap_entry.clap_version,
            clap_wrap::clap_version_t {
                major: 1,
                minor: 1,
                revision: 6,
            }
        ));

        let init_func = clap_entry.init.unwrap();
        let deinit_func = clap_entry.deinit.unwrap();
        let get_factory = clap_entry.get_factory.unwrap();

        let plugin_path = b"/some/path\0".as_ptr().cast();
        let factory_id = b"my.clap.plugin.id\0".as_ptr().cast();

        // load plugin
        let successful = (init_func)(plugin_path);
        if successful {
            let factory = (get_factory)(factory_id);
            // TODO: use factory for something
            println!("plugin factory struct is at {:?}", factory);
            let factory: *const clap_plugin_factory_t = factory.cast();
            let factory = factory.as_ref().unwrap();
            dbg!(factory);
        } else {
            todo!("error handling");
        }

        // cleanup
        deinit_func();
    };
}
