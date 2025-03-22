/*
 * Vapi API
 *
 * API for building voice assistants
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};
use utoipa::OpenApi;


use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, OpenApi)]
pub struct Log {
    /// This is the timestamp at which the log was written.
    #[serde(rename = "time")]
    pub time: String,
    /// This is the unique identifier for the org that this log belongs to.
    #[serde(rename = "orgId")]
    pub org_id: String,
    /// This is the type of the log.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the type of the webhook, given the log is from a webhook.
    #[serde(rename = "webhookType", skip_serializing_if = "Option::is_none")]
    pub webhook_type: Option<String>,
    /// This is the specific resource, relevant only to API logs.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
    /// 'This is how long the request took.
    #[serde(
        rename = "requestDurationSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_duration_seconds: Option<f64>,
    /// This is the timestamp at which the request began.
    #[serde(rename = "requestStartedAt", skip_serializing_if = "Option::is_none")]
    pub request_started_at: Option<String>,
    /// This is the timestamp at which the request finished.
    #[serde(rename = "requestFinishedAt", skip_serializing_if = "Option::is_none")]
    pub request_finished_at: Option<String>,
    /// This is the body of the request.
    #[serde(rename = "requestBody", skip_serializing_if = "Option::is_none")]
    pub request_body: Option<serde_json::Value>,
    /// This is the request method.
    #[serde(rename = "requestHttpMethod", skip_serializing_if = "Option::is_none")]
    pub request_http_method: Option<RequestHttpMethod>,
    /// This is the request URL.
    #[serde(rename = "requestUrl", skip_serializing_if = "Option::is_none")]
    pub request_url: Option<String>,
    /// This is the request path.
    #[serde(rename = "requestPath", skip_serializing_if = "Option::is_none")]
    pub request_path: Option<String>,
    /// This is the request query.
    #[serde(rename = "requestQuery", skip_serializing_if = "Option::is_none")]
    pub request_query: Option<String>,
    /// This the HTTP status code of the response.
    #[serde(rename = "responseHttpCode", skip_serializing_if = "Option::is_none")]
    pub response_http_code: Option<f64>,
    /// This is the request IP address.
    #[serde(rename = "requestIpAddress", skip_serializing_if = "Option::is_none")]
    pub request_ip_address: Option<String>,
    /// This is the origin of the request
    #[serde(rename = "requestOrigin", skip_serializing_if = "Option::is_none")]
    pub request_origin: Option<String>,
    /// This is the body of the response.
    #[serde(rename = "responseBody", skip_serializing_if = "Option::is_none")]
    pub response_body: Option<serde_json::Value>,
    /// These are the headers of the request.
    #[serde(rename = "requestHeaders", skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<serde_json::Value>,
    /// This is the error, if one occurred.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<models::Error>,
    /// This is the ID of the assistant.
    #[serde(rename = "assistantId", skip_serializing_if = "Option::is_none")]
    pub assistant_id: Option<String>,
    /// This is the ID of the phone number.
    #[serde(rename = "phoneNumberId", skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
    /// This is the ID of the customer.
    #[serde(rename = "customerId", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// This is the ID of the squad.
    #[serde(rename = "squadId", skip_serializing_if = "Option::is_none")]
    pub squad_id: Option<String>,
    /// This is the ID of the call.
    #[serde(rename = "callId", skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
}

impl Log {
    pub fn new(time: String, org_id: String, r#type: Type) -> Log {
        Log {
            time,
            org_id,
            r#type,
            webhook_type: None,
            resource: None,
            request_duration_seconds: None,
            request_started_at: None,
            request_finished_at: None,
            request_body: None,
            request_http_method: None,
            request_url: None,
            request_path: None,
            request_query: None,
            response_http_code: None,
            request_ip_address: None,
            request_origin: None,
            response_body: None,
            request_headers: None,
            error: None,
            assistant_id: None,
            phone_number_id: None,
            customer_id: None,
            squad_id: None,
            call_id: None,
        }
    }
}
/// This is the type of the log.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Type {
    #[serde(rename = "API")]
    Api,
    #[serde(rename = "Webhook")]
    Webhook,
    #[serde(rename = "Call")]
    Call,
    #[serde(rename = "Provider")]
    Provider,
}

impl Default for Type {
    fn default() -> Type {
        Self::Api
    }
}
/// This is the specific resource, relevant only to API logs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum Resource {
    #[serde(rename = "org")]
    Org,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "analytics")]
    Analytics,
    #[serde(rename = "credential")]
    Credential,
    #[serde(rename = "phone-number")]
    PhoneNumber,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "voice-library")]
    VoiceLibrary,
    #[serde(rename = "provider")]
    Provider,
    #[serde(rename = "tool")]
    Tool,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "template")]
    Template,
    #[serde(rename = "squad")]
    Squad,
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "metric")]
    Metric,
    #[serde(rename = "log")]
    Log,
}

impl Default for Resource {
    fn default() -> Resource {
        Self::Org
    }
}
/// This is the request method.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, OpenApi)]
pub enum RequestHttpMethod {
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for RequestHttpMethod {
    fn default() -> RequestHttpMethod {
        Self::Post
    }
}
