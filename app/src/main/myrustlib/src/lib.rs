use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub fn inner_rust_greeting(to: &str) -> String {
    "Rust community: Hello ".to_owned() + to
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = CStr::from_ptr(to);
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new(inner_rust_greeting(recipient))
        .unwrap()
        .into_raw()
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn rust_greeting_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    let _ = CString::from_raw(s);
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_myrustapp_RustBindings_greeting(
        env: JNIEnv,
        _: JClass,
        java_pattern: JString,
    ) -> jstring {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        let world = rust_greeting(
            env.get_string(java_pattern)
                .expect("invalid pattern string")
                .as_ptr(),
        );
        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        let output = env
            .new_string(CStr::from_ptr(world).to_str().unwrap())
            .expect("Couldn't create java string!");
        rust_greeting_free(world);

        output.into_inner()
    }
}
