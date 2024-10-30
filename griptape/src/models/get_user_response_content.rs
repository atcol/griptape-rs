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
pub struct GetUserResponseContent {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl GetUserResponseContent {
    pub fn new(user_id: String, created_at: String, updated_at: String) -> GetUserResponseContent {
        GetUserResponseContent {
            user_id,
            created_at,
            updated_at,
        }
    }
}

