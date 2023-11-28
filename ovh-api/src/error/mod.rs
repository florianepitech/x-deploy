pub struct OvhClientError {
    pub request_error: Option<reqwest::Error>,
    pub serialize_error: Option<serde_json::Error>,
    pub wrong_response: Option<String>,
    pub code: Option<u16>,
}

impl OvhClientError {
    pub(crate) fn from_reqwest_error(error: reqwest::Error) -> Self {
        Self {
            request_error: Some(error),
            serialize_error: None,
            wrong_response: None,
            code: None,
        }
    }

    pub(crate) fn from_serialize_error(error: serde_json::Error) -> Self {
        Self {
            request_error: None,
            serialize_error: Some(error),
            wrong_response: None,
            code: None,
        }
    }

    pub(crate) fn from_wrong_response(response: String, code: u16) -> Self {
        Self {
            request_error: None,
            serialize_error: None,
            wrong_response: Some(response),
            code: Some(code),
        }
    }
}
