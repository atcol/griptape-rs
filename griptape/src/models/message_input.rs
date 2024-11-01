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
pub struct MessageInput {
    #[serde(rename = "input")]
    pub input: String,
    #[serde(rename = "output")]
    pub output: String,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl MessageInput {
    pub fn new(input: String, output: String) -> MessageInput {
        MessageInput {
            input,
            output,
            metadata: None,
        }
    }
}

