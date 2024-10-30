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
pub struct CreateStructureRunResponseContent {
    #[serde(rename = "structure_run_id")]
    pub structure_run_id: String,
    #[serde(rename = "structure_id")]
    pub structure_id: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "started_at")]
    pub started_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "completed_at")]
    pub completed_at: String,
    #[serde(rename = "status")]
    pub status: models::StructureRunStatus,
    #[serde(rename = "status_detail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<Option<serde_json::Value>>,
    #[serde(rename = "args")]
    pub args: Vec<String>,
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "env_vars", skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<Vec<models::EnvVar>>,
    #[serde(rename = "output", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub output: Option<Option<serde_json::Value>>,
    #[serde(rename = "output_timestamp", skip_serializing_if = "Option::is_none")]
    pub output_timestamp: Option<f64>,
    #[serde(rename = "deployment_id", skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

impl CreateStructureRunResponseContent {
    pub fn new(structure_run_id: String, structure_id: String, created_by: String, created_at: String, started_at: String, updated_at: String, completed_at: String, status: models::StructureRunStatus, args: Vec<String>) -> CreateStructureRunResponseContent {
        CreateStructureRunResponseContent {
            structure_run_id,
            structure_id,
            created_by,
            created_at,
            started_at,
            updated_at,
            completed_at,
            status,
            status_detail: None,
            args,
            env: None,
            env_vars: None,
            output: None,
            output_timestamp: None,
            deployment_id: None,
        }
    }
}

