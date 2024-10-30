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
pub struct UpdateRulesetResponseContent {
    #[serde(rename = "ruleset_id")]
    pub ruleset_id: String,
    #[serde(rename = "organization_id")]
    pub organization_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "alias")]
    pub alias: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "rule_ids")]
    pub rule_ids: Vec<String>,
    #[serde(rename = "metadata")]
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl UpdateRulesetResponseContent {
    pub fn new(ruleset_id: String, organization_id: String, name: String, description: String, alias: String, created_at: String, updated_at: String, created_by: String, rule_ids: Vec<String>, metadata: std::collections::HashMap<String, serde_json::Value>) -> UpdateRulesetResponseContent {
        UpdateRulesetResponseContent {
            ruleset_id,
            organization_id,
            name,
            description,
            alias,
            created_at,
            updated_at,
            created_by,
            rule_ids,
            metadata,
        }
    }
}

