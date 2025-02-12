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

/// AllocationInnerAssetClassShort : short positions allocation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllocationInnerAssetClassShort {
    #[serde(rename = "STK", skip_serializing_if = "Option::is_none")]
    pub stk: Option<f64>,
    #[serde(rename = "OPT", skip_serializing_if = "Option::is_none")]
    pub opt: Option<f64>,
    #[serde(rename = "FUT", skip_serializing_if = "Option::is_none")]
    pub fut: Option<f64>,
    #[serde(rename = "WAR", skip_serializing_if = "Option::is_none")]
    pub war: Option<f64>,
    #[serde(rename = "BOND", skip_serializing_if = "Option::is_none")]
    pub bond: Option<f64>,
    #[serde(rename = "CASH", skip_serializing_if = "Option::is_none")]
    pub cash: Option<f64>,
}

impl AllocationInnerAssetClassShort {
    /// short positions allocation
    pub fn new() -> AllocationInnerAssetClassShort {
        AllocationInnerAssetClassShort {
            stk: None,
            opt: None,
            fut: None,
            war: None,
            bond: None,
            cash: None,
        }
    }
}

