use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};
use serde_json::json;
use std::error::Error;
use std::fmt;

mod handler;

#[derive(Deserialize, Serialize)]
pub struct ImportResp {
    data: String,
    err: String,
}

#[no_mangle]
pub extern fn _planetr_allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr(); 
    mem::forget(buffer); 
    pointer as *mut c_void
}

#[no_mangle]
pub extern fn _planetr_deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {let _ = Vec::from_raw_parts(pointer, 0, capacity);}
}

#[no_mangle]
pub extern fn _planetr_run_func(req_json: *mut c_char) -> *const c_char {
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
    
    let c_string = CString::new(json_res).expect("CString::new failed");
    return c_string.into_raw();
}

fn _planetr_error(err: &str) -> *const c_char {
    let resp = json! ({"status_code": 500,"body": err,});
    let c_string = CString::new(resp.to_string()).expect("CString::new failed");
    return c_string.into_raw();
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
    fn http_get(self: &Self, url: String) -> Result<String, PlanetrError> {
        unsafe {
            let a_ptr = url.as_ptr();
            let a_len = url.len();
            let ret_ptr = _planetr_host_http_get(a_ptr, a_len);
            if ret_ptr == std::ptr::null::<c_char>() as *mut c_char {
                return Err(PlanetrError::new("internal server error"));
            }
            let c_str: &CStr = CStr::from_ptr(ret_ptr);
            let resp_json = match c_str.to_str(){
                Ok(resp_json) => resp_json,
                Err(err) => return Err(PlanetrError::new(&err.to_string()))
            };
            let json_obj : ImportResp = match serde_json::from_str(resp_json){
                Ok(json_obj) => json_obj,
                Err(err) => return Err(PlanetrError::new(&err.to_string()))  
            };
            if json_obj.err.len() > 0 {
                return Err(PlanetrError::new(&json_obj.err.to_string()))  
            }
            Ok(json_obj.data.to_string())
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
