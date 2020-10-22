/*
 * Substrate API Sidecar v1.
 *
 * Substrate API Sidecar is a REST service that makes it easy to interact with blockchain nodes built using Substrate's FRAME framework.
 *
 * The version of the OpenAPI document: 1.0.0-oas3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionFailedToParse : Error message when Sidecar fails to parse the transaction.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionFailedToParse {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<f32>,
    /// `Failed to parse a tx.`
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
    #[serde(rename = "cause", skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "stack", skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

impl TransactionFailedToParse {
    /// Error message when Sidecar fails to parse the transaction.
    pub fn new() -> TransactionFailedToParse {
        TransactionFailedToParse {
            code: None,
            error: None,
            transaction: None,
            cause: None,
            stack: None,
        }
    }
}

