#![allow(warnings)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/tc-rust.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};
    use libc::{c_char, c_int};
    use crate::tcutil::*;

    #[test]
    fn test_tcutil() {
        let _ = 5 + 5;
    }
}