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
pub struct GetTokenResponseContent {
    #[serde(rename = "access_token")]
    pub access_token: String,
}

impl GetTokenResponseContent {
    pub fn new(access_token: String) -> GetTokenResponseContent {
        GetTokenResponseContent {
            access_token,
        }
    }
}

