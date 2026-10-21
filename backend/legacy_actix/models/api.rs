use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: impl Into<String>, data: T) -> Self {
        ApiResponse {
            success: true,
            message: message.into(),
            data: Some(data),
        }
    }

    #[allow(dead_code)]
    pub fn message(message: impl Into<String>) -> Self {
        ApiResponse {
            success: true,
            message: message.into(),
            data: None,
        }
    }
}
