/*
NOT TO EDIT. 
*/
use std::ffi::{CStr};
use std::mem;
use std::os::raw::{c_char, c_void};
use serde_json::json;
use std::error::Error;
use std::fmt;

mod handler;

#[no_mangle]
pub extern fn _planetr_allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr(); mem::forget(buffer); pointer as *mut c_void
}

#[no_mangle]
pub extern fn _planetr_deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {let _ = Vec::from_raw_parts(pointer, 0, capacity);}
}

#[no_mangle]
pub extern fn _planetr_run_func(req_json: *mut c_char) -> *const u8 {
    let c_str: &CStr = unsafe { CStr::from_ptr(req_json) };
    let str_slice = match c_str.to_str(){
        Ok(str_slice) => str_slice,
        Err(err) => return _planetr_error(&err.to_string())  
    };

    let json_req : handler::InputPayload = match serde_json::from_str(str_slice){
        Ok(json_req) => json_req,
        Err(err) => return _planetr_error(&err.to_string())  
    };

    let ctx = Context{};
    let result = match handler::handle_req(json_req, ctx){
        Ok(result) => result,
        Err(err) => return _planetr_error(&err.to_string())  
    };

    let json_res = match serde_json::to_string(&result){
        Ok(json_res) => json_res,
        Err(err) => return _planetr_error(&err.to_string())  
    };
    json_res.to_string().as_ptr()
}

fn _planetr_error(err: &str) -> *const u8 {
    let resp = json! ({"status_code": 500,"body": err,});
    return resp.to_string().as_ptr()      
}

extern "C" { fn _planetr_host_http_get(a_ptr: *const u8, a_len: usize) -> *mut c_char;}
extern "C" { fn _planetr_host_log(a_ptr: *const u8, a_len: usize);}

pub struct Context {}
#[allow(dead_code)]
impl Context {
    fn log(self: &Self, logstr: String) {
        unsafe {
            let a_ptr = logstr.as_ptr();
            let a_len = logstr.len();
            _planetr_host_log(a_ptr, a_len);
        }
    }
    fn http_get(self: &Self, url: String) -> String {
        unsafe {
            let a_ptr = url.as_ptr();
            let a_len = url.len();
            let _ret = _planetr_host_http_get(a_ptr, a_len);
            let c_str: &CStr = CStr::from_ptr(_ret);
            let str_slice = match c_str.to_str(){
                Ok(str_slice) => str_slice,
                Err(err) => return err.to_string()
            };
            return str_slice.to_string()
        }
    }
}

#[derive(Debug)]
pub struct PlanetrError {
    details: String
}
#[allow(dead_code)]
impl PlanetrError {
    fn new(msg: &str) -> PlanetrError {PlanetrError{details: msg.to_string()}}
}
impl fmt::Display for PlanetrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",self.details)}
}
impl Error for PlanetrError {
    fn description(&self) -> &str {&self.details}
}
