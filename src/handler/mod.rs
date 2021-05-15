use serde_json::{Value};
use crate::PlanetrError;
use crate::PlanetrResponse;

// Function handler to implement
pub fn handle_req(params: Value) -> Result<PlanetrResponse, PlanetrError> {
    
    // -----------------------------
    // Sample code. Edit Below.
    // -----------------------------

    //error condition
    if params["name"] == Value::Null {
        return Err(PlanetrError::new("Name cannot be empty"));
    }

    //convert to upper case and add Hello...
    Ok(PlanetrResponse{
        status_code: 200,
        body : format!("Hello {}", params["name"].to_string().to_ascii_uppercase()),
    })

    // -----------------------------
    // Sample code end.
    // -----------------------------
}
