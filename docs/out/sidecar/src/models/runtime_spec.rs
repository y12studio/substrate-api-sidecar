/*
 * Substrate API Sidecar v1.
 *
 * Substrate API Sidecar is a REST service that makes it easy to interact with blockchain nodes built using Substrate's FRAME framework.
 *
 * The version of the OpenAPI document: 1.0.0-oas3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RuntimeSpec : Version information related to the runtime.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeSpec {
    #[serde(rename = "at", skip_serializing_if = "Option::is_none")]
    pub at: Option<crate::models::BlockIdentifiers>,
    /// The version of the authorship interface. An authoring node will not attempt to author blocks unless this is equal to its native runtime.
    #[serde(rename = "authoringVersion", skip_serializing_if = "Option::is_none")]
    pub authoring_version: Option<String>,
    /// Type of the chain.
    #[serde(rename = "chainType", skip_serializing_if = "Option::is_none")]
    pub chain_type: Option<ChainType>,
    /// Version of the implementation specification. Non-consensus-breaking optimizations are about the only changes that could be made which would result in only the `impl_version` changing. The `impl_version` is set to 0 when `spec_version` is incremented.
    #[serde(rename = "implVersion", skip_serializing_if = "Option::is_none")]
    pub impl_version: Option<String>,
    /// Identifies the different Substrate runtimes.
    #[serde(rename = "specName", skip_serializing_if = "Option::is_none")]
    pub spec_name: Option<String>,
    /// Version of the runtime specification.
    #[serde(rename = "specVersion", skip_serializing_if = "Option::is_none")]
    pub spec_version: Option<String>,
    /// All existing dispatches are fully compatible when this number doesn't change. This number must change when an existing dispatchable (module ID, dispatch ID) is changed, either through an alteration in its user-level semantics, a parameter added/removed/changed, a dispatchable being removed, a module being removed, or a dispatchable/module changing its index.
    #[serde(rename = "transactionVersion", skip_serializing_if = "Option::is_none")]
    pub transaction_version: Option<String>,
    /// Arbitrary properties defined in the chain spec.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl RuntimeSpec {
    /// Version information related to the runtime.
    pub fn new() -> RuntimeSpec {
        RuntimeSpec {
            at: None,
            authoring_version: None,
            chain_type: None,
            impl_version: None,
            spec_name: None,
            spec_version: None,
            transaction_version: None,
            properties: None,
        }
    }
}

/// Type of the chain.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChainType {
    #[serde(rename = "Development")]
    Development,
    #[serde(rename = "Local")]
    Local,
    #[serde(rename = "Live")]
    Live,
}

