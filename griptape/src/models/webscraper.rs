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
pub struct Webscraper {
    #[serde(rename = "webscraper")]
    pub webscraper: Box<models::WebscraperInput>,
}

impl Webscraper {
    pub fn new(webscraper: models::WebscraperInput) -> Webscraper {
        Webscraper {
            webscraper: Box::new(webscraper),
        }
    }
}
