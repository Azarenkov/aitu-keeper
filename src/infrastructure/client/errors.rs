#[derive(serde::Serialize)]
pub struct ResponseError {
    exception: String,
    errorcode: String,
    message: String,
}

enum ResponseErrors {}
