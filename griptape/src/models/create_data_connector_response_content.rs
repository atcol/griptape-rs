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
pub struct CreateDataConnectorResponseContent {
    #[serde(rename = "data_connector_id")]
    pub data_connector_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "config")]
    pub config: Box<models::DataConnectorConfigUnion>,
    #[serde(rename = "data_job_id")]
    pub data_job_id: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "schedule_expression", skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "embedding_model", skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<models::EmbeddingModel>,
    #[serde(rename = "use_default_embedding_model", skip_serializing_if = "Option::is_none")]
    pub use_default_embedding_model: Option<bool>,
}

impl CreateDataConnectorResponseContent {
    pub fn new(data_connector_id: String, name: String, config: models::DataConnectorConfigUnion, data_job_id: String, created_at: String, updated_at: String, created_by: String, r#type: String) -> CreateDataConnectorResponseContent {
        CreateDataConnectorResponseContent {
            data_connector_id,
            name,
            config: Box::new(config),
            data_job_id,
            description: None,
            created_at,
            updated_at,
            created_by,
            schedule_expression: None,
            r#type,
            access_token: None,
            embedding_model: None,
            use_default_embedding_model: None,
        }
    }
}
