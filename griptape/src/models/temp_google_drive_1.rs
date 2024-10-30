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
pub struct TempGoogleDrive1 {
    #[serde(rename = "temp_google_drive")]
    pub temp_google_drive: Box<models::TempGoogleDriveDetail>,
}

impl TempGoogleDrive1 {
    pub fn new(temp_google_drive: models::TempGoogleDriveDetail) -> TempGoogleDrive1 {
        TempGoogleDrive1 {
            temp_google_drive: Box::new(temp_google_drive),
        }
    }
}

