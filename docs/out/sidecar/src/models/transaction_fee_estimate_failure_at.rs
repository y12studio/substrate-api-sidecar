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
pub struct TransactionFeeEstimateFailureAt {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

impl TransactionFeeEstimateFailureAt {
    pub fn new() -> TransactionFeeEstimateFailureAt {
        TransactionFeeEstimateFailureAt {
            hash: None,
        }
    }
}


