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
pub struct CreateRuleRequestContent {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "rule")]
    pub rule: String,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl CreateRuleRequestContent {
    pub fn new(name: String, rule: String) -> CreateRuleRequestContent {
        CreateRuleRequestContent {
            name,
            rule,
            metadata: None,
        }
    }
}
