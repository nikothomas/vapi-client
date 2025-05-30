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
pub struct Condition {
    /// This is the operator you want to use to compare the parameter and value.
    #[serde(rename = "operator")]
    pub operator: Operator,
    /// This is the name of the parameter that you want to check.
    #[serde(rename = "param")]
    pub param: String,
    /// This is the value you want to compare against the parameter.
    #[serde(rename = "value")]
    pub value: serde_json::Value,
}

impl Condition {
    pub fn new(operator: Operator, param: String, value: serde_json::Value) -> Condition {
        Condition {
            operator,
            param,
            value,
        }
    }
}
/// This is the operator you want to use to compare the parameter and value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "neq")]
    Neq,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "gte")]
    Gte,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lte")]
    Lte,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Eq
    }
}

