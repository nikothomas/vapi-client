/*
 * Vapi API
 *
 * Voice AI for developers.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`phone_number_controller_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhoneNumberControllerCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`phone_number_controller_find_all`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhoneNumberControllerFindAllError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`phone_number_controller_find_one`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhoneNumberControllerFindOneError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`phone_number_controller_remove`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhoneNumberControllerRemoveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`phone_number_controller_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhoneNumberControllerUpdateError {
    UnknownValue(serde_json::Value),
}


pub async fn phone_number_controller_create(configuration: &configuration::Configuration, phone_number_controller_create_request: models::PhoneNumberControllerCreateRequest) -> Result<models::PhoneNumber, Error<PhoneNumberControllerCreateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_phone_number_controller_create_request = phone_number_controller_create_request;

    let uri_str = format!("{}/phone-number", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_phone_number_controller_create_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PhoneNumber`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PhoneNumber`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PhoneNumberControllerCreateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn phone_number_controller_find_all(configuration: &configuration::Configuration, limit: Option<f64>, created_at_gt: Option<String>, created_at_lt: Option<String>, created_at_ge: Option<String>, created_at_le: Option<String>, updated_at_gt: Option<String>, updated_at_lt: Option<String>, updated_at_ge: Option<String>, updated_at_le: Option<String>) -> Result<Vec<models::PhoneNumber>, Error<PhoneNumberControllerFindAllError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_limit = limit;
    let p_created_at_gt = created_at_gt;
    let p_created_at_lt = created_at_lt;
    let p_created_at_ge = created_at_ge;
    let p_created_at_le = created_at_le;
    let p_updated_at_gt = updated_at_gt;
    let p_updated_at_lt = updated_at_lt;
    let p_updated_at_ge = updated_at_ge;
    let p_updated_at_le = updated_at_le;

    let uri_str = format!("{}/phone-number", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_created_at_gt {
        req_builder = req_builder.query(&[("createdAtGt", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_created_at_lt {
        req_builder = req_builder.query(&[("createdAtLt", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_created_at_ge {
        req_builder = req_builder.query(&[("createdAtGe", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_created_at_le {
        req_builder = req_builder.query(&[("createdAtLe", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_updated_at_gt {
        req_builder = req_builder.query(&[("updatedAtGt", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_updated_at_lt {
        req_builder = req_builder.query(&[("updatedAtLt", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_updated_at_ge {
        req_builder = req_builder.query(&[("updatedAtGe", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_updated_at_le {
        req_builder = req_builder.query(&[("updatedAtLe", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::PhoneNumber&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::PhoneNumber&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PhoneNumberControllerFindAllError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn phone_number_controller_find_one(configuration: &configuration::Configuration, id: &str) -> Result<models::PhoneNumber, Error<PhoneNumberControllerFindOneError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!("{}/phone-number/{id}", configuration.base_path, id=crate::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PhoneNumber`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PhoneNumber`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PhoneNumberControllerFindOneError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn phone_number_controller_remove(configuration: &configuration::Configuration, id: &str) -> Result<models::PhoneNumber, Error<PhoneNumberControllerRemoveError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!("{}/phone-number/{id}", configuration.base_path, id=crate::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PhoneNumber`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PhoneNumber`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PhoneNumberControllerRemoveError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn phone_number_controller_update(configuration: &configuration::Configuration, id: &str, phone_number_controller_update_request: models::PhoneNumberControllerUpdateRequest) -> Result<models::PhoneNumber, Error<PhoneNumberControllerUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_phone_number_controller_update_request = phone_number_controller_update_request;

    let uri_str = format!("{}/phone-number/{id}", configuration.base_path, id=crate::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_phone_number_controller_update_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PhoneNumber`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PhoneNumber`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PhoneNumberControllerUpdateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

