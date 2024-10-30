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
pub struct DurationPlot {
    #[serde(rename = "seconds_p100", skip_serializing_if = "Option::is_none")]
    pub seconds_p100: Option<f64>,
    #[serde(rename = "seconds_p50", skip_serializing_if = "Option::is_none")]
    pub seconds_p50: Option<f64>,
    #[serde(rename = "seconds_avg", skip_serializing_if = "Option::is_none")]
    pub seconds_avg: Option<f64>,
    #[serde(rename = "timeseries", skip_serializing_if = "Option::is_none")]
    pub timeseries: Option<Vec<models::DurationTimeseriesElement>>,
}

impl DurationPlot {
    pub fn new() -> DurationPlot {
        DurationPlot {
            seconds_p100: None,
            seconds_p50: None,
            seconds_avg: None,
            timeseries: None,
        }
    }
}

