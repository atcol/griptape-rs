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
pub struct Confluence1 {
    #[serde(rename = "confluence")]
    pub confluence: Box<models::ConfluenceDetail>,
}

impl Confluence1 {
    pub fn new(confluence: models::ConfluenceDetail) -> Confluence1 {
        Confluence1 {
            confluence: Box::new(confluence),
        }
    }
}

