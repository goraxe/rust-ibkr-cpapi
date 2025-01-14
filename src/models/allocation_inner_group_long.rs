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

/// AllocationInnerGroupLong : long positions allocation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllocationInnerGroupLong {
    #[serde(rename = "Computers", skip_serializing_if = "Option::is_none")]
    pub computers: Option<f64>,
    #[serde(rename = "Semiconductors", skip_serializing_if = "Option::is_none")]
    pub semiconductors: Option<f64>,
    #[serde(rename = "Others", skip_serializing_if = "Option::is_none")]
    pub others: Option<f64>,
    #[serde(rename = "Chemicals", skip_serializing_if = "Option::is_none")]
    pub chemicals: Option<f64>,
    #[serde(rename = "Apparel", skip_serializing_if = "Option::is_none")]
    pub apparel: Option<f64>,
    #[serde(rename = "Communications", skip_serializing_if = "Option::is_none")]
    pub communications: Option<f64>,
}

impl AllocationInnerGroupLong {
    /// long positions allocation
    pub fn new() -> AllocationInnerGroupLong {
        AllocationInnerGroupLong {
            computers: None,
            semiconductors: None,
            others: None,
            chemicals: None,
            apparel: None,
            communications: None,
        }
    }
}

