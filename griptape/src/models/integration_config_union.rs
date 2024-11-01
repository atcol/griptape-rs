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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntegrationConfigUnion {
    Slack1(Box<models::Slack1>),
}

impl Default for IntegrationConfigUnion {
    fn default() -> Self {
        Self::Slack1(Default::default())
    }
}

