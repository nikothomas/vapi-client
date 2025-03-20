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
pub struct CreateWorkflowDto {
    #[serde(rename = "nodes")]
    pub nodes: Vec<models::WorkflowNodesInner>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "edges")]
    pub edges: Vec<models::Edge>,
}

impl CreateWorkflowDto {
    pub fn new(
        nodes: Vec<models::WorkflowNodesInner>,
        name: String,
        edges: Vec<models::Edge>,
    ) -> CreateWorkflowDto {
        CreateWorkflowDto { nodes, name, edges }
    }
}
