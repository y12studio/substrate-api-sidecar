/*
 * Substrate API Sidecar v1.
 *
 * Substrate API Sidecar is a REST service that makes it easy to interact with blockchain nodes built using Substrate's FRAME framework.
 *
 * The version of the OpenAPI document: 1.0.0-oas3
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `runtime_code_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RuntimeCodeGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `runtime_metadata_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RuntimeMetadataGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `runtime_spec_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RuntimeSpecGetError {
    UnknownValue(serde_json::Value),
}


/// Returns the runtime Wasm blob in hex format.
pub async fn runtime_code_get(configuration: &configuration::Configuration, at: Option<&str>) -> Result<crate::models::RuntimeCode, Error<RuntimeCodeGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/runtime/code", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = at {
        local_var_req_builder = local_var_req_builder.query(&[("at", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RuntimeCodeGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the runtime metadata as a JSON object. Substrate Reference: - FRAME Support: https://crates.parity.io/frame_support/metadata/index.html - Knowledge Base: https://substrate.dev/docs/en/knowledgebase/runtime/metadata
pub async fn runtime_metadata_get(configuration: &configuration::Configuration, at: Option<&str>) -> Result<serde_json::Value, Error<RuntimeMetadataGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/runtime/metadata", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = at {
        local_var_req_builder = local_var_req_builder.query(&[("at", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RuntimeMetadataGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns version information related to the runtime.
pub async fn runtime_spec_get(configuration: &configuration::Configuration, at: Option<&str>) -> Result<crate::models::RuntimeSpec, Error<RuntimeSpecGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/runtime/spec", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = at {
        local_var_req_builder = local_var_req_builder.query(&[("at", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RuntimeSpecGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

