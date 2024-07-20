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
pub struct CcpAccountGet200ResponseAcctListInner {
    /// For multi-account structures each trading account will numbered from 0 to ...
    #[serde(rename = "0", skip_serializing_if = "Option::is_none")]
    pub param_0: Option<String>,
}

impl CcpAccountGet200ResponseAcctListInner {
    pub fn new() -> CcpAccountGet200ResponseAcctListInner {
        CcpAccountGet200ResponseAcctListInner {
            param_0: None,
        }
    }
}

