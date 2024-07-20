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
pub struct LogoutPost200Response {
    /// true means username is still logged in, false means it is not
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<bool>,
}

impl LogoutPost200Response {
    pub fn new() -> LogoutPost200Response {
        LogoutPost200Response {
            confirmed: None,
        }
    }
}
