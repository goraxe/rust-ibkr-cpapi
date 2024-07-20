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
pub struct IserverMarketdataSnapshotGet400Response {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

impl IserverMarketdataSnapshotGet400Response {
    pub fn new() -> IserverMarketdataSnapshotGet400Response {
        IserverMarketdataSnapshotGet400Response {
            error: None,
            status_code: None,
        }
    }
}
