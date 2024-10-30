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
pub struct ListOrganizationsResponseContent {
    #[serde(rename = "organizations", skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<models::OrganizationDetail>>,
}

impl ListOrganizationsResponseContent {
    pub fn new() -> ListOrganizationsResponseContent {
        ListOrganizationsResponseContent {
            organizations: None,
        }
    }
}

