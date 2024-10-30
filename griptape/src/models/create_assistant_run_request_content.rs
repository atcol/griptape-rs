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
pub struct CreateAssistantRunRequestContent {
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "knowledge_base_ids", skip_serializing_if = "Option::is_none")]
    pub knowledge_base_ids: Option<Vec<String>>,
    #[serde(rename = "additional_knowledge_base_ids", skip_serializing_if = "Option::is_none")]
    pub additional_knowledge_base_ids: Option<Vec<String>>,
    #[serde(rename = "ruleset_ids", skip_serializing_if = "Option::is_none")]
    pub ruleset_ids: Option<Vec<String>>,
    #[serde(rename = "additional_ruleset_ids", skip_serializing_if = "Option::is_none")]
    pub additional_ruleset_ids: Option<Vec<String>>,
    #[serde(rename = "structure_ids", skip_serializing_if = "Option::is_none")]
    pub structure_ids: Option<Vec<String>>,
    #[serde(rename = "additional_structure_ids", skip_serializing_if = "Option::is_none")]
    pub additional_structure_ids: Option<Vec<String>>,
    #[serde(rename = "thread_id", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl CreateAssistantRunRequestContent {
    pub fn new() -> CreateAssistantRunRequestContent {
        CreateAssistantRunRequestContent {
            input: None,
            args: None,
            knowledge_base_ids: None,
            additional_knowledge_base_ids: None,
            ruleset_ids: None,
            additional_ruleset_ids: None,
            structure_ids: None,
            additional_structure_ids: None,
            thread_id: None,
            stream: None,
        }
    }
}

