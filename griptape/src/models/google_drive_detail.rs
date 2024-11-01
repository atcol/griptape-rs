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
pub struct GoogleDriveDetail {
    #[serde(rename = "file_ids")]
    pub file_ids: Vec<String>,
}

impl GoogleDriveDetail {
    pub fn new(file_ids: Vec<String>) -> GoogleDriveDetail {
        GoogleDriveDetail {
            file_ids,
        }
    }
}

