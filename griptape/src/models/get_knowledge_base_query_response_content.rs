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
pub struct GetKnowledgeBaseQueryResponseContent {
    #[serde(rename = "knowledge_base_query_id")]
    pub knowledge_base_query_id: String,
    #[serde(rename = "knowledge_base_id")]
    pub knowledge_base_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "query")]
    pub query: String,
    #[serde(rename = "entries")]
    pub entries: Vec<models::Entry>,
}

impl GetKnowledgeBaseQueryResponseContent {
    pub fn new(knowledge_base_query_id: String, knowledge_base_id: String, created_at: String, created_by: String, query: String, entries: Vec<models::Entry>) -> GetKnowledgeBaseQueryResponseContent {
        GetKnowledgeBaseQueryResponseContent {
            knowledge_base_query_id,
            knowledge_base_id,
            created_at,
            created_by,
            query,
            entries,
        }
    }
}

