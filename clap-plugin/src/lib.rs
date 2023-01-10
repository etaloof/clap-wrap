use clap_wrap::{
    clap_host_t, clap_plugin_descriptor_t, clap_plugin_entry_t, clap_plugin_factory,
    clap_plugin_factory_t, clap_plugin_t, clap_version_t,
};
use libc::{c_char, c_void};
use std::ffi::CStr;
use std::ptr::{eq, null_mut};

// This constant corresponds to the `CLAP_VERSION_INIT` macro in `version.h`.
pub const CLAP_VERSION_INIT: clap_version_t = clap_version_t {
    major: 1,
    minor: 1,
    revision: 6,
};

#[no_mangle]
pub static clap_entry: clap_plugin_entry_t = clap_plugin_entry_t {
    clap_version: CLAP_VERSION_INIT,
    init: Some(init),
    deinit: Some(deinit),
    get_factory: Some(get_factory),
};

unsafe extern "C" fn init(_: *const c_char) -> bool {
    true
}

unsafe extern "C" fn deinit() {}

unsafe extern "C" fn get_factory(factory_id: *const c_char) -> *const c_void {
    let equal = unsafe { libc::strcmp(factory_id, CLAP_PLUGIN_FACTORY_ID.as_ptr()) };
    if equal == 0 {
        std::ptr::addr_of!(plugin_factory).cast()
    } else {
        null_mut()
    }
}

#[no_mangle]
pub static plugin_factory: clap_plugin_factory_t = clap_plugin_factory_t {
    get_plugin_count: Some(get_plugin_count),
    get_plugin_descriptor: Some(get_plugin_descriptor),
    create_plugin: Some(create_plugin),
};

unsafe extern "C" fn get_plugin_count(factory: *const clap_plugin_factory) -> u32 {
    todo!()
}

unsafe extern "C" fn get_plugin_descriptor(
    factory: *const clap_plugin_factory,
    index: u32,
) -> *const clap_plugin_descriptor_t {
    todo!()
}

unsafe extern "C" fn create_plugin(
    factory: *const clap_plugin_factory,
    host: *const clap_host_t,
    plugin_id: *const c_char,
) -> *const clap_plugin_t {
    todo!()
}

#[no_mangle]
pub static CLAP_PLUGIN_FACTORY_ID: [::std::os::raw::c_char; 20usize] = unsafe {
    /// TODO: Rewrite using std::array::map once it is stable
    std::mem::transmute(*b"my.clap.plugin.id\0\0\0")
};
