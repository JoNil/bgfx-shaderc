use std::{
    ffi::{c_int, c_void},
    ptr,
};

#[link(name = "bgfx_shaderc")]
unsafe extern "C" {
    fn cpp_main(argc: c_int, argv: *const *const c_void);
}

fn main() {
    unsafe { cpp_main(0, ptr::null()) };
}
