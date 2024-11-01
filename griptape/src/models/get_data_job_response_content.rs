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
pub struct GetDataJobResponseContent {
    #[serde(rename = "data_job_id")]
    pub data_job_id: String,
    #[serde(rename = "data_connector_id")]
    pub data_connector_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "completed_at", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "status")]
    pub status: models::DataJobStatus,
    #[serde(rename = "bytes_ingested", skip_serializing_if = "Option::is_none")]
    pub bytes_ingested: Option<f64>,
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::Error>>,
}

impl GetDataJobResponseContent {
    pub fn new(data_job_id: String, data_connector_id: String, created_at: String, created_by: String, status: models::DataJobStatus) -> GetDataJobResponseContent {
        GetDataJobResponseContent {
            data_job_id,
            data_connector_id,
            created_at,
            completed_at: None,
            created_by,
            status,
            bytes_ingested: None,
            errors: None,
        }
    }
}

