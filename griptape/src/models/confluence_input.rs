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
pub struct ConfluenceInput {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "atlassian_email")]
    pub atlassian_email: String,
    #[serde(rename = "atlassian_api_token")]
    pub atlassian_api_token: String,
}

impl ConfluenceInput {
    pub fn new(domain: String, atlassian_email: String, atlassian_api_token: String) -> ConfluenceInput {
        ConfluenceInput {
            domain,
            atlassian_email,
            atlassian_api_token,
        }
    }
}

