/* automatically generated by rust-bindgen */

extern "C" {
    #[doc = " Writes the constant string `text` to the log, with priority `prio` and tag"]
    #[doc = " `tag`."]
    pub fn __android_log_write(
        prio: ::std::os::raw::c_int,
        tag: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
