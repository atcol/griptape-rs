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
pub struct TransformInput {
    #[serde(rename = "structure")]
    pub structure: Box<models::StructureConnectorInput>,
}

impl TransformInput {
    pub fn new(structure: models::StructureConnectorInput) -> TransformInput {
        TransformInput {
            structure: Box::new(structure),
        }
    }
}

