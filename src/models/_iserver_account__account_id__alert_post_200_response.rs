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
pub struct IserverAccountAccountIdAlertPost200Response {
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i32>,
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "order_status", skip_serializing_if = "Option::is_none")]
    pub order_status: Option<String>,
    #[serde(rename = "warning_message", skip_serializing_if = "Option::is_none")]
    pub warning_message: Option<String>,
}

impl IserverAccountAccountIdAlertPost200Response {
    pub fn new() -> IserverAccountAccountIdAlertPost200Response {
        IserverAccountAccountIdAlertPost200Response {
            request_id: None,
            order_id: None,
            success: None,
            text: None,
            order_status: None,
            warning_message: None,
        }
    }
}
