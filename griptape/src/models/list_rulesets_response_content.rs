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
pub struct ListRulesetsResponseContent {
    #[serde(rename = "rulesets")]
    pub rulesets: Vec<models::RulesetDetail>,
    #[serde(rename = "pagination")]
    pub pagination: Box<models::Pagination>,
}

impl ListRulesetsResponseContent {
    pub fn new(rulesets: Vec<models::RulesetDetail>, pagination: models::Pagination) -> ListRulesetsResponseContent {
        ListRulesetsResponseContent {
            rulesets,
            pagination: Box::new(pagination),
        }
    }
}
