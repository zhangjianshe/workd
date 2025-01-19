use serde::Serialize;
#[derive(Serialize, Debug, Clone)]
pub struct ApiResponse<T: Serialize> {
    pub code: String,
    pub message: String,
    pub data: Option<Box<T>>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: Box<T>) -> ApiResponse<T> {
        ApiResponse {
            code: "200".into(),
            message: String::from("success"),
            data: Some(data),
        }
    }
    pub fn fail(code: String, msg: Option<String>) -> ApiResponse<String> {
        ApiResponse {
            code,
            message: match msg {
                Some(msg) => msg,
                _ => "".into(),
            },
            data: None,
        }
    }
}
