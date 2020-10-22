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
pub struct AccountVestingInfo {
    #[serde(rename = "at", skip_serializing_if = "Option::is_none")]
    pub at: Option<crate::models::BlockIdentifiers>,
    #[serde(rename = "vesting", skip_serializing_if = "Option::is_none")]
    pub vesting: Option<crate::models::VestingSchedule>,
}

impl AccountVestingInfo {
    pub fn new() -> AccountVestingInfo {
        AccountVestingInfo {
            at: None,
            vesting: None,
        }
    }
}


