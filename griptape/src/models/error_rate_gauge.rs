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
pub struct ErrorRateGauge {
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    #[serde(rename = "error_type_counts", skip_serializing_if = "Option::is_none")]
    pub error_type_counts: Option<Vec<models::ErrorTypeCount>>,
}

impl ErrorRateGauge {
    pub fn new() -> ErrorRateGauge {
        ErrorRateGauge {
            rate: None,
            error_type_counts: None,
        }
    }
}

