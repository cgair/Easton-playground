#![allow(unused)]
/**
 * Idioms that may be useful when doing FFI:
 * [1] Idiomatic Errors - Error handling with integer codes and sentinel return values (such as NULL pointers)
 * [2] Accepting Strings with minimal unsafe code
 * [3] Passing Strings to FFI functions
 */

// ///////////////// //
// Accepting Strings //
// ///////////////// //
// The strings used in C have different behaviours to those used in Rust:
// * C strings are null-terminated while Rust strings store their length
// * C strings can contain any arbitrary non-zero byte while Rust strings must be UTF-8
// * C strings are accessed and manipulated using unsafe pointer operations while interactions with Rust strings go through safe methods

// The Rust standard library comes with C equivalents of Rust's String and &str called CString and &CStr (allows us to work with borrowed data, meaning passing strings between Rust and C is a zero-cost operation), 
// that allow us to avoid a lot of the complexity and unsafe code involved in converting between C strings and Rust strings.

pub mod unsafe_module {
    use libc::{c_char, c_int};
    use std::ffi::CStr;

    /// Log a message at the specified level.
    ///
    /// # Safety
    ///
    /// It is the caller's guarantee to ensure `msg`:
    ///
    /// - is not a null pointer
    /// - points to valid, initialized data
    /// - points to memory ending in a null byte
    /// - won't be mutated for the duration of this function call
    #[no_mangle]
    pub unsafe extern "C" fn mylib_log(
        msg: *const c_char,
        level: c_int
    ) {
        // let level: /* */ = match level { /* ... */ };

        // SAFETY: The caller has already guaranteed this is okay (see the
        // `# Safety` section of the doc-comment).
        let msg_str = match CStr::from_ptr(msg).to_str() {
            Ok(s) => s,
            Err(_) => {
                println!("FFI string conversion failed");
                return;
            }
        };
        // crate::log(msg_str, level);
        println!("{msg_str}");
    }
    // The example up ensure that: 1) The unsafe block is as small as possible. 2) The pointer with an "untracked" lifetime becomes a "tracked" shared reference

    // Consider an alternative, where the string is actually copied:
    use std::ffi::CString;
    #[no_mangle]
    pub extern "C" fn mylib_log_ugly(msg: *const libc::c_char, level: libc::c_int) {
        // DO NOT USE THIS CODE.
        // IT IS UGLY, VERBOSE, AND CONTAINS A SUBTLE BUG.

        // let level: /* */ = match level { /* ... */ };

        let msg_len = unsafe { /* SAFETY: strlen is what it is, I guess? */
            libc::strlen(msg)   // <https://www.gnu.org/software/libc/manual/html_node/String-Length.html>
        };

        let mut msg_data = Vec::with_capacity(msg_len + 1);
        let msg_cstr = unsafe {            
            // SAFETY: copying from a foreign pointer expected to live
            // for the entire stack frame into owned memory
            std::ptr::copy_nonoverlapping(msg, msg_data.as_mut_ptr(), msg_len);  // The bug here is a simple mistake in pointer arithmetic: 
                                                                                                // the string was copied, all msg_len bytes of it. However, the NUL terminator at the end was not.

            /* This method can be useful for situations in which the vector is serving as a buffer for other code, particularly over FFI */                                                                                                
            msg_data.set_len(msg_len + 1);  // The Vector then had its size set to the length of the zero padded string -- rather than resized to it, which could have added a zero at the end. 
                                                    // As a result, the last byte in the Vector is uninitialized memory.

            // <https://stackoverflow.com/questions/48308759/how-do-i-convert-a-vect-to-a-vecu-without-copying-the-vector>
            let data = msg_data.iter().map(|&d| d as u8).collect::<Vec<_>>();

            CString::from_vec_with_nul(data).unwrap()   
        };

        let msg_str: String = unsafe {
            match msg_cstr.into_string() {
                Ok(s) => s,
                Err(_) => {
                    println!("FFI string conversion failed");
                    return;
                }
            }
        };

        // crate::log(&msg_str, level);
        println!("{msg_str}");
    }
}






// ///////////////// //
// Accepting Strings //
// ///////////////// //

pub mod another_unsafe_module {

    // extern "C" {
    //     fn seterr(message: *const libc::c_char);
    //     fn geterr(buffer: *mut libc::c_char, size: libc::c_int) -> libc::c_int;
    // }


    pub fn report_error_to_ffi<S: Into<String>>(
        err: S
    ) -> Result<(), std::ffi::NulError>{
        let c_err = std::ffi::CString::new(err.into())?;

        unsafe {
            // SAFETY: calling an FFI whose documentation says the pointer is
            // const, so no modification should occur
            seterr(c_err.as_ptr());
        }

        Ok(())
        // The lifetime of c_err continues until here
    }

    pub fn report_error<S: Into<String>>(err: S) -> Result<(), std::ffi::NulError> {
        unsafe {
            // SAFETY: whoops, this contains a dangling pointer!
            seterr(std::ffi::CString::new(err.into())?.as_ptr());
        }
        Ok(())
    }

    fn seterr(message: *const i8) {
        unsafe {
            println!("{}", *message);
        }
    }
}

#[test]
fn test_accept() {
    let s = String::from("value");
    another_unsafe_module::report_error_to_ffi(s.clone());
    another_unsafe_module::report_error(s).unwrap();
}