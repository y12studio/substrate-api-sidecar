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
pub struct RuntimeCode {
    #[serde(rename = "at", skip_serializing_if = "Option::is_none")]
    pub at: Option<crate::models::BlockIdentifiers>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl RuntimeCode {
    pub fn new() -> RuntimeCode {
        RuntimeCode {
            at: None,
            code: None,
        }
    }
}


