use rocket::Request;
use rocket_contrib::{JSON, Value};

/// 404 Not found catcher
#[error(404)]
fn not_found(req: &Request) -> JSON<Value> {
    let resp = match req.content_type() {
        // Check if it's application/json typed
        Some(ref ctxt) if !ctxt.is_json() => {
            json!({
                "success": false,
                "message": format!("Sorry we only supports JSON requests, not '{}'.", ctxt)
            })
        },
        _ => {
            json!({
                "success": false,
                "message": format!("'{}' is an invalid URL.", req.uri())
            })
        }
    };
    JSON(resp)
}

/// 400 Bad Request catcher
#[error(400)]
fn bad_request() -> JSON<Value> {
    JSON(json!({
        "success": false,
        "message": "The request could not be understood by the server due to malformed syntax."
    }))
}

/// 401 Unauthorized
#[error(401)]
fn unauthorized() -> JSON<Value> {
    JSON(json!({
        "success": false,
        "message": "Missing or invalid authentication."
    }))
}

/// 403 Forbidden
#[error(403)]
fn forbidden(req: &Request) -> JSON<Value> {
    JSON(json!({
        "success": false,
        "message": format!("'{}' is forbidden for user", req.uri())
    }))
}

