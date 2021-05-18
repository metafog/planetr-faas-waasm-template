use serde::{Deserialize, Serialize};
use planetr::wasm::Context;
use planetr::wasm::PlanetrError;

#[derive(Deserialize, Serialize)]
pub struct InputPayload {
    // Define input payload JSON structure HERE
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct OuputPayload {
    // Define output payload JSON structure HERE
    hello: String,
}

pub fn handle_req(args: InputPayload, ctx: Context) -> Result<OuputPayload, PlanetrError> {
    
    // -----------------------------
    // Sample code. Edit Below.
    // -----------------------------

    //error condition
    if args.name == "" {
        return Err(PlanetrError::new("missing name field in request"));
    }

    //HTTP request feature...
    /*
    let resp = match ctx.http_get("https://planetr.io"){
        Ok(resp) => resp,
        Err(err) => return Err(err)
    };
    */

    //Log
    ctx.log(format!("Name={}", args.name));

    //convert to upper case and respond Hello...
    Ok(OuputPayload{
        hello: format!("Hello {}", args.name).to_string(),
    })
}
