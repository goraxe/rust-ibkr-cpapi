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
pub struct IserverAccountAccountIdAlertActivatePostRequest {
    /// alert id(order id)
    #[serde(rename = "alertId", skip_serializing_if = "Option::is_none")]
    pub alert_id: Option<i32>,
    /// 1 to activate, 0 to deactivate
    #[serde(rename = "alertActive", skip_serializing_if = "Option::is_none")]
    pub alert_active: Option<i32>,
}

impl IserverAccountAccountIdAlertActivatePostRequest {
    pub fn new() -> IserverAccountAccountIdAlertActivatePostRequest {
        IserverAccountAccountIdAlertActivatePostRequest {
            alert_id: None,
            alert_active: None,
        }
    }
}

