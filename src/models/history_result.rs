/*
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryResult {
    #[serde(rename = "bars", skip_serializing_if = "Option::is_none")]
    pub bars: Option<Box<models::HistoryResultBars>>,
}

impl HistoryResult {
    pub fn new() -> HistoryResult {
        HistoryResult {
            bars: None,
        }
    }
}

