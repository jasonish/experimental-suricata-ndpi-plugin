use std::ffi::c_void;

use ndpi_sys as _;

unsafe extern "C" {
    fn NdpiPluginRegister() -> *const c_void;
}

#[no_mangle]
pub extern "C" fn SCPluginRegister() -> *const c_void {
    unsafe { NdpiPluginRegister() }
}
