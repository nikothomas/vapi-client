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
pub struct GcpKey {
    /// This is the type of the key. Most likely, this is \"service_account\".
    #[serde(rename = "type")]
    pub r#type: String,
    /// This is the ID of the Google Cloud project associated with this key.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// This is the unique identifier for the private key.
    #[serde(rename = "privateKeyId")]
    pub private_key_id: String,
    /// This is the private key in PEM format.  Note: This is not returned in the API.
    #[serde(rename = "privateKey")]
    pub private_key: String,
    /// This is the email address associated with the service account.
    #[serde(rename = "clientEmail")]
    pub client_email: String,
    /// This is the unique identifier for the client.
    #[serde(rename = "clientId")]
    pub client_id: String,
    /// This is the URI for the auth provider's authorization endpoint.
    #[serde(rename = "authUri")]
    pub auth_uri: String,
    /// This is the URI for the auth provider's token endpoint.
    #[serde(rename = "tokenUri")]
    pub token_uri: String,
    /// This is the URL of the public x509 certificate for the auth provider.
    #[serde(rename = "authProviderX509CertUrl")]
    pub auth_provider_x509_cert_url: String,
    /// This is the URL of the public x509 certificate for the client.
    #[serde(rename = "clientX509CertUrl")]
    pub client_x509_cert_url: String,
    /// This is the domain associated with the universe this service account belongs to.
    #[serde(rename = "universeDomain")]
    pub universe_domain: String,
}

impl GcpKey {
    pub fn new(
        r#type: String,
        project_id: String,
        private_key_id: String,
        private_key: String,
        client_email: String,
        client_id: String,
        auth_uri: String,
        token_uri: String,
        auth_provider_x509_cert_url: String,
        client_x509_cert_url: String,
        universe_domain: String,
    ) -> GcpKey {
        GcpKey {
            r#type,
            project_id,
            private_key_id,
            private_key,
            client_email,
            client_id,
            auth_uri,
            token_uri,
            auth_provider_x509_cert_url,
            client_x509_cert_url,
            universe_domain,
        }
    }
}
