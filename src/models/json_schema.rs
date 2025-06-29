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
pub struct JsonSchema {
    /// This is the type of output you'd like.  `string`, `number`, `integer`, `boolean` are the primitive types and should be obvious.  `array` and `object` are more interesting and quite powerful. They allow you to define nested structures.  For `array`, you can define the schema of the items in the array using the `items` property.  For `object`, you can define the properties of the object using the `properties` property.
    #[serde(rename = "type")]
    pub r#type: TypeTrue,
    /// This is required if the type is \"array\". This is the schema of the items in the array.  This is of type JsonSchema. However, Swagger doesn't support circular references.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<serde_json::Value>,
    /// This is required if the type is \"object\". This specifies the properties of the object.  This is a map of string to JsonSchema. However, Swagger doesn't support circular references.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// This is the description to help the model understand what it needs to output.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// This is a list of properties that are required.  This only makes sense if the type is \"object\".
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    /// This the value that will be used in filling the property.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// This the target variable that will be filled with the value of this property.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// This array specifies the allowed values that can be used to restrict the output of the model.
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
}

impl JsonSchema {
    pub fn new(r#type: TypeTrue) -> JsonSchema {
        JsonSchema {
            r#type,
            items: None,
            properties: None,
            description: None,
            required: None,
            value: None,
            target: None,
            r#enum: None,
        }
    }
}
/// This is the type of output you'd like.  `string`, `number`, `integer`, `boolean` are the primitive types and should be obvious.  `array` and `object` are more interesting and quite powerful. They allow you to define nested structures.  For `array`, you can define the schema of the items in the array using the `items` property.  For `object`, you can define the properties of the object using the `properties` property.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeTrue {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "object")]
    Object,
}

impl Default for TypeTrue {
    fn default() -> TypeTrue {
        Self::String
    }
}
