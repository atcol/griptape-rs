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
pub struct Github1 {
    #[serde(rename = "github")]
    pub github: Box<models::GithubCodeSourceInput>,
}

impl Github1 {
    pub fn new(github: models::GithubCodeSourceInput) -> Github1 {
        Github1 {
            github: Box::new(github),
        }
    }
}

