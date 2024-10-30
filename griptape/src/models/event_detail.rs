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
pub struct EventDetail {
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "structure_run_id")]
    pub structure_run_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "payload", deserialize_with = "Option::deserialize")]
    pub payload: Option<serde_json::Value>,
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl EventDetail {
    pub fn new(event_id: String, structure_run_id: String, created_at: String, origin: String, payload: Option<serde_json::Value>, timestamp: f64, r#type: String) -> EventDetail {
        EventDetail {
            event_id,
            structure_run_id,
            created_at,
            origin,
            payload,
            timestamp,
            r#type,
        }
    }
}

