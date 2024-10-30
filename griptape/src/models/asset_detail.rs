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
pub struct AssetDetail {
    #[serde(rename = "organization_id")]
    pub organization_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
}

impl AssetDetail {
    pub fn new(organization_id: String, name: String, created_at: String, updated_at: String, created_by: String) -> AssetDetail {
        AssetDetail {
            organization_id,
            name,
            created_at,
            updated_at,
            created_by,
        }
    }
}

