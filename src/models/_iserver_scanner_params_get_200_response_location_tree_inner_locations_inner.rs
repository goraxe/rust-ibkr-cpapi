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
pub struct IserverScannerParamsGet200ResponseLocationTreeInnerLocationsInner {
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl IserverScannerParamsGet200ResponseLocationTreeInnerLocationsInner {
    pub fn new() -> IserverScannerParamsGet200ResponseLocationTreeInnerLocationsInner {
        IserverScannerParamsGet200ResponseLocationTreeInnerLocationsInner {
            display_name: None,
            r#type: None,
        }
    }
}
