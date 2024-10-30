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
pub struct ObservabilityEvent {
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "attributes", deserialize_with = "Option::deserialize")]
    pub attributes: Option<serde_json::Value>,
}

impl ObservabilityEvent {
    pub fn new(timestamp: String, name: String, attributes: Option<serde_json::Value>) -> ObservabilityEvent {
        ObservabilityEvent {
            timestamp,
            name,
            attributes,
        }
    }
}
