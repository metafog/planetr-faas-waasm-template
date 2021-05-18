
use std::ffi::{CString};
use std::os::raw::{c_char};

mod handler;

#[no_mangle]
pub extern fn _planetr_run_func(req_json: *mut c_char) -> *const c_char {
    let reqjs = match planetr::wasm::wasm_parse_func_args(req_json){
        Ok(reqjs) => reqjs,
        Err(err) => return planetr::wasm::wasm_error(&err.to_string())  
    };

    let json_req : handler::InputPayload = match serde_json::from_str(&reqjs){
        Ok(json_req) => json_req,
        Err(err) => return planetr::wasm::wasm_error(&err.to_string())  
    };

    let ctx = planetr::wasm::Context{};
    let result = match handler::handle_req(json_req, ctx){
        Ok(result) => result,
        Err(err) => return planetr::wasm::wasm_error(&err.to_string())  
    };

    let json_res = match serde_json::to_string(&result){
        Ok(json_res) => json_res,
        Err(err) => return planetr::wasm::wasm_error(&err.to_string())  
    };
    
    let c_string = CString::new(json_res).expect("CString::new failed");
    return c_string.into_raw();
}
