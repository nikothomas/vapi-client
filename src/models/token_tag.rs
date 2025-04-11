/*
 * Vapi API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// TokenTag : This is the tag for the token. It represents its scope.
/// This is the tag for the token. It represents its scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TokenTag {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,

}

impl std::fmt::Display for TokenTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Private => write!(f, "private"),
            Self::Public => write!(f, "public"),
        }
    }
}

impl Default for TokenTag {
    fn default() -> TokenTag {
        Self::Private
    }
}

