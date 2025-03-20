/*
 * Vapi API
 *
 * API for building voice assistants
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use std::sync::Arc;

use async_trait::async_trait;
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

use super::{configuration, Error};
use crate::{
    apis::{ContentType, ResponseContent},
    models,
};

#[async_trait]
pub trait TestSuitesApi: Send + Sync {
    /// POST /test-suite
    async fn test_suite_controller_create<'create_test_suite_dto>(
        &self,
        create_test_suite_dto: models::CreateTestSuiteDto,
    ) -> Result<models::TestSuite, Error<TestSuiteControllerCreateError>>;

    /// GET /test-suite
    async fn test_suite_controller_find_all_paginated<
        'page,
        'sort_order,
        'limit,
        'created_at_gt,
        'created_at_lt,
        'created_at_ge,
        'created_at_le,
        'updated_at_gt,
        'updated_at_lt,
        'updated_at_ge,
        'updated_at_le,
    >(
        &self,
        page: Option<f64>,
        sort_order: Option<&'sort_order str>,
        limit: Option<f64>,
        created_at_gt: Option<String>,
        created_at_lt: Option<String>,
        created_at_ge: Option<String>,
        created_at_le: Option<String>,
        updated_at_gt: Option<String>,
        updated_at_lt: Option<String>,
        updated_at_ge: Option<String>,
        updated_at_le: Option<String>,
    ) -> Result<models::TestSuitesPaginatedResponse, Error<TestSuiteControllerFindAllPaginatedError>>;

    /// GET /test-suite/{id}
    async fn test_suite_controller_find_one<'id>(
        &self,
        id: &'id str,
    ) -> Result<models::TestSuite, Error<TestSuiteControllerFindOneError>>;

    /// DELETE /test-suite/{id}
    async fn test_suite_controller_remove<'id>(
        &self,
        id: &'id str,
    ) -> Result<models::TestSuite, Error<TestSuiteControllerRemoveError>>;

    /// PATCH /test-suite/{id}
    async fn test_suite_controller_update<'id, 'update_test_suite_dto>(
        &self,
        id: &'id str,
        update_test_suite_dto: models::UpdateTestSuiteDto,
    ) -> Result<models::TestSuite, Error<TestSuiteControllerUpdateError>>;
}

pub struct TestSuitesApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl TestSuitesApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

#[async_trait]
impl TestSuitesApi for TestSuitesApiClient {
    async fn test_suite_controller_create<'create_test_suite_dto>(
        &self,
        create_test_suite_dto: models::CreateTestSuiteDto,
    ) -> Result<models::TestSuite, Error<TestSuiteControllerCreateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/test-suite", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&create_test_suite_dto);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `models::TestSuite`",
                    )))
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::TestSuite`"
                    ))))
                }
            }
        } else {
            let local_var_entity: Option<TestSuiteControllerCreateError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn test_suite_controller_find_all_paginated<
        'page,
        'sort_order,
        'limit,
        'created_at_gt,
        'created_at_lt,
        'created_at_ge,
        'created_at_le,
        'updated_at_gt,
        'updated_at_lt,
        'updated_at_ge,
        'updated_at_le,
    >(
        &self,
        page: Option<f64>,
        sort_order: Option<&'sort_order str>,
        limit: Option<f64>,
        created_at_gt: Option<String>,
        created_at_lt: Option<String>,
        created_at_ge: Option<String>,
        created_at_le: Option<String>,
        updated_at_gt: Option<String>,
        updated_at_lt: Option<String>,
        updated_at_ge: Option<String>,
        updated_at_le: Option<String>,
    ) -> Result<models::TestSuitesPaginatedResponse, Error<TestSuiteControllerFindAllPaginatedError>>
    {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/test-suite", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = page {
            local_var_req_builder =
                local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = sort_order {
            local_var_req_builder =
                local_var_req_builder.query(&[("sortOrder", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = limit {
            local_var_req_builder =
                local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = created_at_gt {
            local_var_req_builder =
                local_var_req_builder.query(&[("createdAtGt", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = created_at_lt {
            local_var_req_builder =
                local_var_req_builder.query(&[("createdAtLt", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = created_at_ge {
            local_var_req_builder =
                local_var_req_builder.query(&[("createdAtGe", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = created_at_le {
            local_var_req_builder =
                local_var_req_builder.query(&[("createdAtLe", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = updated_at_gt {
            local_var_req_builder =
                local_var_req_builder.query(&[("updatedAtGt", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = updated_at_lt {
            local_var_req_builder =
                local_var_req_builder.query(&[("updatedAtLt", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = updated_at_ge {
            local_var_req_builder =
                local_var_req_builder.query(&[("updatedAtGe", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = updated_at_le {
            local_var_req_builder =
                local_var_req_builder.query(&[("updatedAtLe", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `models::TestSuitesPaginatedResponse`",
                    )))
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::TestSuitesPaginatedResponse`"
                    ))))
                }
            }
        } else {
            let local_var_entity: Option<TestSuiteControllerFindAllPaginatedError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn test_suite_controller_find_one<'id>(
        &self,
        id: &'id str,
    ) -> Result<models::TestSuite, Error<TestSuiteControllerFindOneError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/test-suite/{id}",
            local_var_configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `models::TestSuite`",
                    )))
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::TestSuite`"
                    ))))
                }
            }
        } else {
            let local_var_entity: Option<TestSuiteControllerFindOneError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn test_suite_controller_remove<'id>(
        &self,
        id: &'id str,
    ) -> Result<models::TestSuite, Error<TestSuiteControllerRemoveError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/test-suite/{id}",
            local_var_configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `models::TestSuite`",
                    )))
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::TestSuite`"
                    ))))
                }
            }
        } else {
            let local_var_entity: Option<TestSuiteControllerRemoveError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    async fn test_suite_controller_update<'id, 'update_test_suite_dto>(
        &self,
        id: &'id str,
        update_test_suite_dto: models::UpdateTestSuiteDto,
    ) -> Result<models::TestSuite, Error<TestSuiteControllerUpdateError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/test-suite/{id}",
            local_var_configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&update_test_suite_dto);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content_type = local_var_resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream");
        let local_var_content_type = super::ContentType::from(local_var_content_type);
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            match local_var_content_type {
                ContentType::Json => serde_json::from_str(&local_var_content).map_err(Error::from),
                ContentType::Text => {
                    return Err(Error::from(serde_json::Error::custom(
                        "Received `text/plain` content type response that cannot be converted to \
                         `models::TestSuite`",
                    )))
                }
                ContentType::Unsupported(local_var_unknown_type) => {
                    return Err(Error::from(serde_json::Error::custom(format!(
                        "Received `{local_var_unknown_type}` content type response that cannot be \
                         converted to `models::TestSuite`"
                    ))))
                }
            }
        } else {
            let local_var_entity: Option<TestSuiteControllerUpdateError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }
}

/// struct for typed errors of method [`test_suite_controller_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestSuiteControllerCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`test_suite_controller_find_all_paginated`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestSuiteControllerFindAllPaginatedError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`test_suite_controller_find_one`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestSuiteControllerFindOneError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`test_suite_controller_remove`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestSuiteControllerRemoveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`test_suite_controller_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestSuiteControllerUpdateError {
    UnknownValue(serde_json::Value),
}
