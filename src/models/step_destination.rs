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
pub struct StepDestination {
    #[serde(rename = "type")]
    pub r#type: Type,
    /// This is an optional array of conditions that must be met for this destination to be triggered. If empty, this is the default destination that the step transfers to.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::BlockStartMessageConditionsInner>>,
    #[serde(rename = "stepName")]
    pub step_name: String,
}

impl StepDestination {
    pub fn new(r#type: Type, step_name: String) -> StepDestination {
        StepDestination {
            r#type,
            conditions: None,
            step_name,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "step")]
    Step,
}

impl Default for Type {
    fn default() -> Type {
        Self::Step
    }
}
