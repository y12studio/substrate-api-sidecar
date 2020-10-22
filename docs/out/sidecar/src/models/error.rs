/*
 * Substrate API Sidecar v1.
 *
 * Substrate API Sidecar is a REST service that makes it easy to interact with blockchain nodes built using Substrate's FRAME framework.
 *
 * The version of the OpenAPI document: 1.0.0-oas3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<f32>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "stack", skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

impl Error {
    pub fn new() -> Error {
        Error {
            code: None,
            message: None,
            stack: None,
        }
    }
}


