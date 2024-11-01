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
pub struct CreateAssistantRunResponseContent {
    #[serde(rename = "assistant_run_id")]
    pub assistant_run_id: String,
    #[serde(rename = "assistant_id")]
    pub assistant_id: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "completed_at")]
    pub completed_at: String,
    #[serde(rename = "status")]
    pub status: models::AssistantRunStatus,
    #[serde(rename = "status_detail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<Option<serde_json::Value>>,
    #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "args")]
    pub args: Vec<String>,
    #[serde(rename = "output", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub output: Option<Option<serde_json::Value>>,
    #[serde(rename = "thread_id", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(rename = "knowledge_base_ids")]
    pub knowledge_base_ids: Vec<String>,
    #[serde(rename = "ruleset_ids")]
    pub ruleset_ids: Vec<String>,
    #[serde(rename = "stream")]
    pub stream: bool,
    #[serde(rename = "structure_ids")]
    pub structure_ids: Vec<String>,
}

impl CreateAssistantRunResponseContent {
    pub fn new(assistant_run_id: String, assistant_id: String, created_by: String, created_at: String, updated_at: String, completed_at: String, status: models::AssistantRunStatus, args: Vec<String>, knowledge_base_ids: Vec<String>, ruleset_ids: Vec<String>, stream: bool, structure_ids: Vec<String>) -> CreateAssistantRunResponseContent {
        CreateAssistantRunResponseContent {
            assistant_run_id,
            assistant_id,
            created_by,
            created_at,
            updated_at,
            completed_at,
            status,
            status_detail: None,
            input: None,
            args,
            output: None,
            thread_id: None,
            knowledge_base_ids,
            ruleset_ids,
            stream,
            structure_ids,
        }
    }
}

