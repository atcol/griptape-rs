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
pub struct ListSpansResponseContent {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<f64>,
    #[serde(rename = "spans")]
    pub spans: Vec<models::SpanDetail>,
}

impl ListSpansResponseContent {
    pub fn new(spans: Vec<models::SpanDetail>) -> ListSpansResponseContent {
        ListSpansResponseContent {
            page: None,
            spans,
        }
    }
}
