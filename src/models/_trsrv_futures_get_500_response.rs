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
pub struct TrsrvFuturesGet500Response {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl TrsrvFuturesGet500Response {
    pub fn new() -> TrsrvFuturesGet500Response {
        TrsrvFuturesGet500Response {
            error: None,
        }
    }
}

