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
pub struct UpdateCustomKnowledgeBaseDto {
    /// This is where the knowledge base request will be sent.  Request Example:  POST https://{server.url} Content-Type: application/json  {   \"messsage\": {     \"type\": \"knowledge-base-request\",     \"messages\": [       {         \"role\": \"user\",         \"content\": \"Why is ocean blue?\"       }     ],     ...other metadata about the call...   } }  Response Expected: ``` {   \"message\": {      \"role\": \"assistant\",      \"content\": \"The ocean is blue because water absorbs everything but blue.\",   }, // YOU CAN RETURN THE EXACT RESPONSE TO SPEAK   \"documents\": [     {       \"content\": \"The ocean is blue primarily because water absorbs colors in the red part of the light spectrum and scatters the blue light, making it more visible to our eyes.\",       \"similarity\": 1     },     {       \"content\": \"Blue light is scattered more by the water molecules than other colors, enhancing the blue appearance of the ocean.\",       \"similarity\": .5     }   ] // OR, YOU CAN RETURN AN ARRAY OF DOCUMENTS THAT WILL BE SENT TO THE MODEL } ```
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<models::Server>,
}

impl UpdateCustomKnowledgeBaseDto {
    pub fn new() -> UpdateCustomKnowledgeBaseDto {
        UpdateCustomKnowledgeBaseDto { server: None }
    }
}
