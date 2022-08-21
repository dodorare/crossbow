#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
pub const HOST_TAG: &str = "windows-x86_64";

#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
pub const HOST_TAG: &str = "windows";

#[cfg(target_os = "linux")]
pub const HOST_TAG: &str = "linux-x86_64";

#[cfg(target_os = "macos")]
pub const HOST_TAG: &str = "darwin-x86_64";

pub const NDK_GLUE_EXTRA_CODE: &str = r#"
#[no_mangle]
#[cfg(target_os = "android")]
unsafe extern "C" fn ANativeActivity_onCreate(
    activity: *mut std::os::raw::c_void,
    saved_state: *mut std::os::raw::c_void,
    saved_state_size: usize,
) {
    crossbow::ndk_glue::init(
        activity as _,
        saved_state as _,
        saved_state_size as _,
        main,
    );
}
"#;

// TODO: Fix this.
pub const SOKOL_EXTRA_CODE: &str = r##"
mod cargo_apk_glue_code {
    #[no_mangle]
    pub unsafe extern "C" fn ANativeActivity_onCreate(
        activity: *mut std::ffi::c_void,
        saved_state: *mut std::ffi::c_void,
        saved_state_size: usize,
    ) {
        crossbow::ndk_glue::init(
            activity as _,
            saved_state as _,
            saved_state_size as _,
            super::main,
        );
        let _ = super::main();
    }
    #[no_mangle]
    pub extern "C" fn quad_main() {
        let _ = super::main();
    }
}
"##;
