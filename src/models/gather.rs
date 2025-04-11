/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gather {
    #[serde(rename = "output")]
    pub output: models::JsonSchema,
    /// This is whether or not the workflow should read back the gathered data to the user, and ask about its correctness.
    #[serde(rename = "confirmContent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub confirm_content: Option<Option<bool>>,
    /// This is a list of hooks for a task. Each hook is a list of tasks to run on a trigger (such as on start, on failure, etc). Only Say is supported for now.
    #[serde(rename = "hooks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Option<Vec<models::Hook>>>,
    /// This is the number of times we should try to gather the information from the user before we failover to the fail path. An example of this would be a user refusing to give their phone number for privacy reasons, and then going down a different path on account of this
    #[serde(rename = "maxRetries", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<Option<f64>>,
    /// This is a liquid templating string. On the first call to Gather, the template will be filled out with variables from the context, and will be spoken verbatim to the user. An example would be \"Base on your zipcode, it looks like you could be in one of these counties: {{ counties | join: \", \" }}. Which one do you live in?\"
    #[serde(rename = "literalTemplate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub literal_template: Option<Option<String>>,
    #[serde(rename = "name")]
    pub name: String,
    /// This is for metadata you want to store on the task.
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<std::collections::HashMap<String, serde_json::Value>>>,
}

impl Gather {
    pub fn new(output: models::JsonSchema, name: String) -> Gather {
        Gather {
            output,
            confirm_content: None,
            hooks: None,
            max_retries: None,
            literal_template: None,
            name,
            metadata: None,
        }
    }
}

