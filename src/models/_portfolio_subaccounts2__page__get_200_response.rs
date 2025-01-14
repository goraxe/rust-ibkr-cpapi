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
pub struct PortfolioSubaccounts2PageGet200Response {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::PortfolioSubaccounts2PageGet200ResponseMetadata>>,
    #[serde(rename = "subaccounts", skip_serializing_if = "Option::is_none")]
    pub subaccounts: Option<Vec<models::PortfolioSubaccounts2PageGet200ResponseSubaccountsInner>>,
}

impl PortfolioSubaccounts2PageGet200Response {
    pub fn new() -> PortfolioSubaccounts2PageGet200Response {
        PortfolioSubaccounts2PageGet200Response {
            metadata: None,
            subaccounts: None,
        }
    }
}

