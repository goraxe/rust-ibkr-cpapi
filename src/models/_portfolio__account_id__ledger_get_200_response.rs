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
pub struct PortfolioAccountIdLedgerGet200Response {
    #[serde(rename = "BASE", skip_serializing_if = "Option::is_none")]
    pub base: Option<Box<models::Ledger>>,
}

impl PortfolioAccountIdLedgerGet200Response {
    pub fn new() -> PortfolioAccountIdLedgerGet200Response {
        PortfolioAccountIdLedgerGet200Response {
            base: None,
        }
    }
}

