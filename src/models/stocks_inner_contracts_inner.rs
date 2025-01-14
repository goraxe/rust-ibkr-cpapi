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
pub struct StocksInnerContractsInner {
    /// conid of the stock contract
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i32>,
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
}

impl StocksInnerContractsInner {
    pub fn new() -> StocksInnerContractsInner {
        StocksInnerContractsInner {
            conid: None,
            exchange: None,
        }
    }
}

