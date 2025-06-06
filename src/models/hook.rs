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
pub struct Hook {
    #[serde(rename = "on")]
    pub on: On,
    #[serde(rename = "do")]
    pub r#do: Vec<models::SayHook>,
}

impl Hook {
    pub fn new(on: On, r#do: Vec<models::SayHook>) -> Hook {
        Hook {
            on,
            r#do,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum On {
    #[serde(rename = "task.start")]
    TaskPeriodStart,
    #[serde(rename = "task.output.confirmation")]
    TaskPeriodOutputPeriodConfirmation,
    #[serde(rename = "task.delayed")]
    TaskPeriodDelayed,
}

impl Default for On {
    fn default() -> On {
        Self::TaskPeriodStart
    }
}

