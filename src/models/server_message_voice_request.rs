/*
 * Vapi API
 *
 * Voice AI for developers.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerMessageVoiceRequest {
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<models::ServerMessageAssistantRequestPhoneNumber>,
    /// This is the type of the message. \"voice-request\" is sent when using `assistant.voice={ \"type\": \"custom-voice\" }`.  Here is what the request will look like:  POST https://{assistant.voice.server.url} Content-Type: application/json  {   \"messsage\": {     \"type\": \"voice-request\",     \"text\": \"Hello, world!\",     \"sampleRate\": 24000,     ...other metadata about the call...   } }  The expected response is 1-channel 16-bit raw PCM audio at the sample rate specified in the request. Here is how the response will be piped to the transport: ``` response.on('data', (chunk: Buffer) => {   outputStream.write(chunk); }); ```
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is the timestamp of when the message was sent in milliseconds since Unix Epoch.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// This is a live version of the `call.artifact`.  This matches what is stored on `call.artifact` after the call.
    #[serde(rename = "artifact", skip_serializing_if = "Option::is_none")]
    pub artifact: Option<models::Artifact>,
    /// This is the assistant that is currently active. This is provided for convenience.  This matches one of the following: - `call.assistant`, - `call.assistantId`, - `call.squad[n].assistant`, - `call.squad[n].assistantId`, - `call.squadId->[n].assistant`, - `call.squadId->[n].assistantId`.
    #[serde(rename = "assistant", skip_serializing_if = "Option::is_none")]
    pub assistant: Option<models::CreateAssistantDto>,
    /// This is the customer associated with the call.  This matches one of the following: - `call.customer`, - `call.customerId`.
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<models::CreateCustomerDto>,
    /// This is the call object.  This matches what was returned in POST /call.  Note: This might get stale during the call. To get the latest call object, especially after the call is ended, use GET /call/:id.
    #[serde(rename = "call", skip_serializing_if = "Option::is_none")]
    pub call: Option<models::Call>,
    /// This is the text to be synthesized.
    #[serde(rename = "text")]
    pub text: String,
    /// This is the sample rate to be synthesized.
    #[serde(rename = "sampleRate")]
    pub sample_rate: f64,
}

impl ServerMessageVoiceRequest {
    pub fn new(r#type: Type, text: String, sample_rate: f64) -> ServerMessageVoiceRequest {
        ServerMessageVoiceRequest {
            phone_number: None,
            r#type,
            timestamp: None,
            artifact: None,
            assistant: None,
            customer: None,
            call: None,
            text,
            sample_rate,
        }
    }
}
/// This is the type of the message. \"voice-request\" is sent when using `assistant.voice={ \"type\": \"custom-voice\" }`.  Here is what the request will look like:  POST https://{assistant.voice.server.url} Content-Type: application/json  {   \"messsage\": {     \"type\": \"voice-request\",     \"text\": \"Hello, world!\",     \"sampleRate\": 24000,     ...other metadata about the call...   } }  The expected response is 1-channel 16-bit raw PCM audio at the sample rate specified in the request. Here is how the response will be piped to the transport: ``` response.on('data', (chunk: Buffer) => {   outputStream.write(chunk); }); ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "voice-request")]
    VoiceRequest,
}

impl Default for Type {
    fn default() -> Type {
        Self::VoiceRequest
    }
}

