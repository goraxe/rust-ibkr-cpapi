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
pub struct FyiDisclaimerTypecodePut200Response {
    #[serde(rename = "T", skip_serializing_if = "Option::is_none")]
    pub t: Option<i32>,
    #[serde(rename = "V", skip_serializing_if = "Option::is_none")]
    pub v: Option<i32>,
}

impl FyiDisclaimerTypecodePut200Response {
    pub fn new() -> FyiDisclaimerTypecodePut200Response {
        FyiDisclaimerTypecodePut200Response {
            t: None,
            v: None,
        }
    }
}
