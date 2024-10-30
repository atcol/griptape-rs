/*
 * Griptape Cloud
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023-09-19
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListKnowledgeBasesResponseContent {
    #[serde(rename = "knowledge_bases", skip_serializing_if = "Option::is_none")]
    pub knowledge_bases: Option<Vec<models::KnowledgeBaseDetail>>,
}

impl ListKnowledgeBasesResponseContent {
    pub fn new() -> ListKnowledgeBasesResponseContent {
        ListKnowledgeBasesResponseContent {
            knowledge_bases: None,
        }
    }
}

