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


/// struct for typed errors of method `dryrun_transaction`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DryrunTransactionError {
    Status500(crate::models::TransactionFailure),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `fee_estimate_transaction`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FeeEstimateTransactionError {
    Status500(crate::models::TransactionFeeEstimateFailure),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_transaction_material`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransactionMaterialError {
    Status400(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `submit_transaction`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitTransactionError {
    Status400(crate::models::TransactionFailure),
    UnknownValue(serde_json::Value),
}


/// Use the dryrun call to practice submission of a transaction.
pub async fn dryrun_transaction(configuration: &configuration::Configuration, transaction: crate::models::Transaction) -> Result<crate::models::TransactionDryRun, Error<DryrunTransactionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/transaction/dry-run", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&transaction);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DryrunTransactionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Send a serialized transaction and receive back a naive fee estimate. Note: `partialFee` does not include any tips that you may add to increase a transaction's priority. See the reference on `compute_fee`. Replaces `/tx/fee-estimate` from versions < v1.0.0. Substrate Reference: - `RuntimeDispatchInfo`: https://crates.parity.io/pallet_transaction_payment_rpc_runtime_api/struct.RuntimeDispatchInfo.html - `query_info`: https://crates.parity.io/pallet_transaction_payment/struct.Module.html#method.query_info - `compute_fee`: https://crates.parity.io/pallet_transaction_payment/struct.Module.html#method.compute_fee
pub async fn fee_estimate_transaction(configuration: &configuration::Configuration, transaction: crate::models::Transaction) -> Result<crate::models::TransactionFeeEstimate, Error<FeeEstimateTransactionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/transaction/fee-estimate", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&transaction);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FeeEstimateTransactionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the material that is universal to constructing any signed transaction offline. Replaces `/tx/artifacts` from versions < v1.0.0.
pub async fn get_transaction_material(configuration: &configuration::Configuration, at: Option<&str>, no_meta: Option<bool>) -> Result<crate::models::TransactionMaterial, Error<GetTransactionMaterialError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/transaction/material", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = at {
        local_var_req_builder = local_var_req_builder.query(&[("at", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = no_meta {
        local_var_req_builder = local_var_req_builder.query(&[("noMeta", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetTransactionMaterialError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Accepts a valid signed extrinsic. Replaces `/tx` from versions < v1.0.0.
pub async fn submit_transaction(configuration: &configuration::Configuration, transaction: crate::models::Transaction) -> Result<crate::models::TransactionSuccess, Error<SubmitTransactionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/transaction", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&transaction);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SubmitTransactionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
