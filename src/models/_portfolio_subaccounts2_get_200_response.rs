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
pub struct PortfolioSubaccounts2Get200Response {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::PortfolioSubaccounts2Get200ResponseMetadata>>,
    #[serde(rename = "subaccounts", skip_serializing_if = "Option::is_none")]
    pub subaccounts: Option<Vec<models::PortfolioSubaccounts2Get200ResponseSubaccountsInner>>,
}

impl PortfolioSubaccounts2Get200Response {
    pub fn new() -> PortfolioSubaccounts2Get200Response {
        PortfolioSubaccounts2Get200Response {
            metadata: None,
            subaccounts: None,
        }
    }
}

