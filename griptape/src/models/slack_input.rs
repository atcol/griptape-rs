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
pub struct SlackInput {
    #[serde(rename = "app_name")]
    pub app_name: String,
    #[serde(rename = "app_display_name")]
    pub app_display_name: String,
    #[serde(rename = "app_description")]
    pub app_description: String,
    #[serde(rename = "bot_token_secret_ref", skip_serializing_if = "Option::is_none")]
    pub bot_token_secret_ref: Option<String>,
    #[serde(rename = "signing_secret_secret_ref", skip_serializing_if = "Option::is_none")]
    pub signing_secret_secret_ref: Option<String>,
}

impl SlackInput {
    pub fn new(app_name: String, app_display_name: String, app_description: String) -> SlackInput {
        SlackInput {
            app_name,
            app_display_name,
            app_description,
            bot_token_secret_ref: None,
            signing_secret_secret_ref: None,
        }
    }
}

