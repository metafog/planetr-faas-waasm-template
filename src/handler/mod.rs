use serde::{Deserialize, Serialize};
use crate::Context;
use crate::PlanetrError;

#[derive(Deserialize, Serialize)]
pub struct InputPayload {
    // Define input payload JSON structure HERE
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct OuputPayload {
    // Define output payload JSON structure HERE
    body: String,
}

pub fn handle_req(args: InputPayload, ctx: Context) -> Result<OuputPayload, PlanetrError> {
    
    // -----------------------------
    // Sample code. Edit Below.
    // -----------------------------

    //error condition
    if args.name == "" {
        return Err(PlanetrError::new("missing name field in request"));
    }

    ctx.log(format!("Name={}", args.name));

    //convert to upper case and add Hello...
    Ok(OuputPayload{
        body: format!("Hello {}", args.name).to_string(),
    })
}
