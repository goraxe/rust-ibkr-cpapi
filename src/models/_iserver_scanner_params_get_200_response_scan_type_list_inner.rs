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
pub struct IserverScannerParamsGet200ResponseScanTypeListInner {
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
    pub instruments: Option<Vec<String>>,
}

impl IserverScannerParamsGet200ResponseScanTypeListInner {
    pub fn new() -> IserverScannerParamsGet200ResponseScanTypeListInner {
        IserverScannerParamsGet200ResponseScanTypeListInner {
            display_name: None,
            code: None,
            instruments: None,
        }
    }
}

