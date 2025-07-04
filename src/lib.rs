use std::{
    env,
    ffi::{CString, c_int, c_void},
    iter,
};

#[link(name = "bgfx_shaderc")]
unsafe extern "C" {
    fn cpp_main(argc: c_int, argv: *const *const c_void) -> i32;
}

pub fn shaderc(args: &[&str]) -> i32 {
    let prog = env::current_exe().unwrap();
    let args = iter::once(prog.to_string_lossy().into_owned())
        .chain(args.iter().map(|s| s.to_string()))
        .map(|s| CString::new(s).unwrap())
        .collect::<Vec<_>>();
    let args = args
        .iter()
        .map(|s| s.as_c_str().as_ptr())
        .collect::<Vec<_>>();

    unsafe { cpp_main(args.len() as _, args.as_ptr() as _) }
}
