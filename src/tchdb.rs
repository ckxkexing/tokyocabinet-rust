//
// Copyright 2022 Kexing Chen
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
#![allow(warnings)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/tc-rust.rs"));

use libc::{c_char, c_int, c_long, c_void};
use std::error::Error;
use std::ffi::{CStr, CString};

// chenkx: custom struct in safe wrapper
pub struct TCH(pub *mut TCHDB);
impl TCH {
    pub fn tchdbnew() -> Option<Self> {
        let res = unsafe{ tchdbnew() };
        if res.is_null(){
            None
        }
        else {
            Some(TCH(res))
        }
    }
    pub fn tchdbopen(&self, path:CString , mode:c_int) -> bool {
        unsafe{tchdbopen(self.0, path.as_ptr(), mode)}
    }
    pub fn tchdbput2(&self, key:CString, value:CString) -> bool{
        unsafe {tchdbput2(self.0, key.as_ptr(), value.as_ptr()) }
    }
    pub fn tchdbget2(&self, key:CString) -> String{
       let v2 = unsafe {tchdbget2(self.0, key.as_ptr())};
       let v2_cstr = unsafe {CStr::from_ptr(v2)};
       String::from(v2_cstr.to_str().unwrap())
    }
    pub fn tchdbdel(&self) {
        unsafe {tchdbdel(self.0) }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};
    use libc::{c_char, c_int};
    use crate::tchdb::*;
    fn str2cstr(str:&str) -> CString {
        CString::new(str).unwrap()
    }

    #[test]
    fn test_new_del() {
        unsafe {
            let db = tchdbnew();
            assert!(!db.is_null());
            tchdbdel(db);
        }
    }

    #[test]
    // chenkx: The unsafe interface is used directly
    fn test_create_tch(){
        unsafe {
            let tchdb = tchdbnew();

            // let cpath = str2cchar("./casket.tch");
            let cpath = str2cstr("./casket.tch");

            let res = tchdbopen(tchdb, cpath.as_ptr(), (HDBOWRITER | HDBOCREAT) as c_int);
            assert_eq!(HDBOWRITER, 2);
            assert_eq!(res, true);

            let err = !tchdbput2(tchdb, str2cstr("foo").as_ptr(), str2cstr("value").as_ptr()) ||
                !tchdbput2(tchdb, str2cstr("bar").as_ptr(), str2cstr("step").as_ptr()) ||
                !tchdbput2(tchdb, str2cstr("baz").as_ptr(), str2cstr("jump").as_ptr());

            assert_eq!(err, false);

            // Query
            let v2 = tchdbget2(tchdb, str2cstr("foo").as_ptr());
            let v2_cstr = CStr::from_ptr(v2);
            println!("{:?}",  v2_cstr);                 // "value"
            println!("{}", v2_cstr.to_str().unwrap());  //  value
            assert_eq!(res, true);

            // Iterator
            tchdbiterinit(tchdb);
            while let key = tchdbiternext2(tchdb){
                if key.is_null(){break;}
                let v2 = tchdbget2(tchdb, key);
                let v2_cstr = CStr::from_ptr(v2);
                println!("traverse :: {:?}:{:?}", CStr::from_ptr(key), v2_cstr);
            }
            /*
                output:
                    traverse :: "foo":"value"
                    traverse :: "bar":"step"
                    traverse :: "baz":"jump"
            */
            tchdbdel(tchdb);

            /*
            *   Start to test Read
            */

            let tchdb = tchdbnew();

            // let cpath = str2cchar("./casket.tch");
            let cpath = str2cstr("./casket.tch");

            let res = tchdbopen(tchdb, cpath.as_ptr(), HDBOREADER as c_int);

            // Query
            let v2 = tchdbget2(tchdb, str2cstr("foo").as_ptr());
            let v2_cstr = CStr::from_ptr(v2);
            println!("{:?}",  v2_cstr);                 // "value"
            println!("{}", v2_cstr.to_str().unwrap());  // value
            assert_eq!(res, true);

            // Iterator
            tchdbiterinit(tchdb);
            while let key = tchdbiternext2(tchdb){
                if key.is_null(){break;}
                let v2 = tchdbget2(tchdb, key);
                let v2_cstr = CStr::from_ptr(v2);
                println!("traverse :: {:?}:{:?}", CStr::from_ptr(key), v2_cstr);
            }

            tchdbdel(tchdb);
        }
    }

    #[test]
    // chenkx: Test custom Struct and Methods
    fn test_TCH(){
        let db = TCH::tchdbnew().unwrap();
        let path = str2cstr("./casket2.tch");
        db.tchdbopen(path, (HDBOWRITER | HDBOCREAT) as c_int);
        db.tchdbput2(str2cstr("foo"), str2cstr("value"));
        let v = db.tchdbget2(str2cstr("foo"));
        assert_eq!(v, String::from("value"));
        db.tchdbdel();
    }
}