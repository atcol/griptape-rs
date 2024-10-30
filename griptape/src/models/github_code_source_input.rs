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
pub struct GithubCodeSourceInput {
    #[serde(rename = "commit_sha", skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}

impl GithubCodeSourceInput {
    pub fn new() -> GithubCodeSourceInput {
        GithubCodeSourceInput {
            commit_sha: None,
            access_token: None,
        }
    }
}

