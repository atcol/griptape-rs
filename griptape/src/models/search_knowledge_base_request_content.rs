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
pub struct SearchKnowledgeBaseRequestContent {
    #[serde(rename = "query")]
    pub query: String,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    #[serde(rename = "include_vectors", skip_serializing_if = "Option::is_none")]
    pub include_vectors: Option<bool>,
    #[serde(rename = "distance_metric", skip_serializing_if = "Option::is_none")]
    pub distance_metric: Option<String>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl SearchKnowledgeBaseRequestContent {
    pub fn new(query: String) -> SearchKnowledgeBaseRequestContent {
        SearchKnowledgeBaseRequestContent {
            query,
            count: None,
            include_vectors: None,
            distance_metric: None,
            filter: None,
        }
    }
}

