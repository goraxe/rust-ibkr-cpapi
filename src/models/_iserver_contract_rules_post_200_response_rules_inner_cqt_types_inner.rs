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
pub struct IserverContractRulesPost200ResponseRulesInnerCqtTypesInner {
    /// order types that support cash quantity trades
    #[serde(rename = "0", skip_serializing_if = "Option::is_none")]
    pub param_0: Option<String>,
}

impl IserverContractRulesPost200ResponseRulesInnerCqtTypesInner {
    pub fn new() -> IserverContractRulesPost200ResponseRulesInnerCqtTypesInner {
        IserverContractRulesPost200ResponseRulesInnerCqtTypesInner {
            param_0: None,
        }
    }
}

