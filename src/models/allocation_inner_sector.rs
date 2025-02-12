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

/// AllocationInnerSector : portfolio allocation by sector
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllocationInnerSector {
    #[serde(rename = "long", skip_serializing_if = "Option::is_none")]
    pub long: Option<Box<models::AllocationInnerSectorLong>>,
    #[serde(rename = "short", skip_serializing_if = "Option::is_none")]
    pub short: Option<Box<models::AllocationInnerSectorShort>>,
}

impl AllocationInnerSector {
    /// portfolio allocation by sector
    pub fn new() -> AllocationInnerSector {
        AllocationInnerSector {
            long: None,
            short: None,
        }
    }
}

