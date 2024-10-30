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
pub struct Github {
    #[serde(rename = "github")]
    pub github: Box<models::GithubCodeSource>,
}

impl Github {
    pub fn new(github: models::GithubCodeSource) -> Github {
        Github {
            github: Box::new(github),
        }
    }
}
