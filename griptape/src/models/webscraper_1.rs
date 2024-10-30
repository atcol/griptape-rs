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
pub struct Webscraper1 {
    #[serde(rename = "webscraper")]
    pub webscraper: Box<models::WebscraperDetail>,
}

impl Webscraper1 {
    pub fn new(webscraper: models::WebscraperDetail) -> Webscraper1 {
        Webscraper1 {
            webscraper: Box::new(webscraper),
        }
    }
}
