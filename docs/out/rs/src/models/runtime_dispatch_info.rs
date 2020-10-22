/*
 * Substrate API Sidecar v1.
 *
 * Substrate API Sidecar is a REST service that makes it easy to interact with blockchain nodes built using Substrate's FRAME framework.
 *
 * The version of the OpenAPI document: 1.0.0-oas3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RuntimeDispatchInfo : RuntimeDispatchInfo for the transaction. Includes the `partialFee`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeDispatchInfo {
    /// Extrinsic weight.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    /// Extrinsic class.
    #[serde(rename = "class", skip_serializing_if = "Option::is_none")]
    pub class: Option<Class>,
    /// The _inclusion fee_ of a transaction, i.e. the minimum fee required for a transaction. Includes weight and encoded length fees, but does not have access to any signed extensions, e.g. the `tip`.
    #[serde(rename = "partialFee", skip_serializing_if = "Option::is_none")]
    pub partial_fee: Option<String>,
}

impl RuntimeDispatchInfo {
    /// RuntimeDispatchInfo for the transaction. Includes the `partialFee`.
    pub fn new() -> RuntimeDispatchInfo {
        RuntimeDispatchInfo {
            weight: None,
            class: None,
            partial_fee: None,
        }
    }
}

/// Extrinsic class.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Class {
    #[serde(rename = "Normal")]
    Normal,
    #[serde(rename = "Operational")]
    Operational,
    #[serde(rename = "Mandatory")]
    Mandatory,
}
