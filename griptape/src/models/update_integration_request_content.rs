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
pub struct UpdateIntegrationRequestContent {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::IntegrationConfigInputUnion>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::IntegrationType>,
    #[serde(rename = "structure_ids", skip_serializing_if = "Option::is_none")]
    pub structure_ids: Option<Vec<String>>,
    #[serde(rename = "assistant_ids", skip_serializing_if = "Option::is_none")]
    pub assistant_ids: Option<Vec<String>>,
}

impl UpdateIntegrationRequestContent {
    pub fn new() -> UpdateIntegrationRequestContent {
        UpdateIntegrationRequestContent {
            name: None,
            description: None,
            config: None,
            r#type: None,
            structure_ids: None,
            assistant_ids: None,
        }
    }
}
