/* automatically generated by rust-bindgen */

extern "C" {
    pub fn A64HookFunction(
        symbol: *mut ::std::os::raw::c_void,
        replace: *mut ::std::os::raw::c_void,
        result: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn A64HookFunctionV(
        symbol: *mut ::std::os::raw::c_void,
        replace: *mut ::std::os::raw::c_void,
        rwx: *mut ::std::os::raw::c_void,
        rwx_size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
