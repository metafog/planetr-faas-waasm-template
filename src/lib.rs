/*
PLEASE DO NOT EDIT. 
*/
use std::ffi::{CStr};
use std::mem;
use std::os::raw::{c_char, c_void};
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

mod handler;

//Handler Response struct
#[derive(Deserialize, Serialize, Clone)]
pub struct PlanetrResponse {
    #[serde(rename = "statusCode")]
    status_code: u16,
    body: String,
}

//Handler Custome Error struct
#[derive(Debug)]
pub struct PlanetrError {
    details: String
}
impl PlanetrError {
    fn new(msg: &str) -> PlanetrError {
        PlanetrError{details: msg.to_string()}
    }
}
impl fmt::Display for PlanetrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}
impl Error for PlanetrError {
    fn description(&self) -> &str {
        &self.details
    }
}

// WASM interface handler
// Allocate memory in wasm memory space
#[no_mangle]
pub extern fn planetr_allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    pointer as *mut c_void
}

// WASM interface handler
// De-allocate memory in wasm memory space
#[no_mangle]
pub extern fn planetr_deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

// WASM interface handler
// entrypoint function to call from the host
#[no_mangle]
pub extern fn planetr_run_func(req_json: *mut c_char) -> *const u8 {
    let c_str: &CStr = unsafe { CStr::from_ptr(req_json) };
    let str_slice: &str = c_str.to_str().unwrap();
    let json_req = match serde_json::from_str(str_slice){
        Ok(json_req) => json_req,
        Err(err) => return planetr_error(err.to_string())  
    };
    let result = match handler::handle_req(json_req){
        Ok(result) => result,
        Err(err) => return planetr_error(err.to_string())  
    };
    let json_res = match serde_json::to_string(&result){
        Ok(json_res) => json_res,
        Err(err) => return planetr_error(err.to_string())  
    };
    json_res.to_string().as_ptr()
}

// utility function to create error response
fn planetr_error(err: String) -> *const u8 {
    let resp = json! ({
        "status_code": 500,
        "body": err,
    });
    return resp.to_string().as_ptr()      
}