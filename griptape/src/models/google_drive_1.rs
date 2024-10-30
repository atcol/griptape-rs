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
pub struct GoogleDrive1 {
    #[serde(rename = "google_drive")]
    pub google_drive: Box<models::GoogleDriveDetail>,
}

impl GoogleDrive1 {
    pub fn new(google_drive: models::GoogleDriveDetail) -> GoogleDrive1 {
        GoogleDrive1 {
            google_drive: Box::new(google_drive),
        }
    }
}
