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

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCustomKnowledgeBaseDto {
    /// This knowledge base is bring your own knowledge base implementation.
    #[serde(rename = "provider")]
    pub provider: Provider,
    /// This is where the knowledge base request will be sent.  Request Example:  POST https://{server.url} Content-Type: application/json  {   \"messsage\": {     \"type\": \"knowledge-base-request\",     \"messages\": [       {         \"role\": \"user\",         \"content\": \"Why is ocean blue?\"       }     ],     ...other metadata about the call...   } }  Response Expected: ``` {   \"message\": {      \"role\": \"assistant\",      \"content\": \"The ocean is blue because water absorbs everything but blue.\",   }, // YOU CAN RETURN THE EXACT RESPONSE TO SPEAK   \"documents\": [     {       \"content\": \"The ocean is blue primarily because water absorbs colors in the red part of the light spectrum and scatters the blue light, making it more visible to our eyes.\",       \"similarity\": 1     },     {       \"content\": \"Blue light is scattered more by the water molecules than other colors, enhancing the blue appearance of the ocean.\",       \"similarity\": .5     }   ] // OR, YOU CAN RETURN AN ARRAY OF DOCUMENTS THAT WILL BE SENT TO THE MODEL } ```
    #[serde(rename = "server")]
    pub server: models::Server,
}

impl CreateCustomKnowledgeBaseDto {
    pub fn new(provider: Provider, server: models::Server) -> CreateCustomKnowledgeBaseDto {
        CreateCustomKnowledgeBaseDto { provider, server }
    }
}
/// This knowledge base is bring your own knowledge base implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Provider {
    #[serde(rename = "custom-knowledge-base")]
    CustomKnowledgeBase,
}

impl Default for Provider {
    fn default() -> Provider {
        Self::CustomKnowledgeBase
    }
}
