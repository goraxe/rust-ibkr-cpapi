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
pub struct IserverContractRulesPost200Response {
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<models::IserverContractRulesPost200ResponseRulesInner>>,
}

impl IserverContractRulesPost200Response {
    pub fn new() -> IserverContractRulesPost200Response {
        IserverContractRulesPost200Response {
            rules: None,
        }
    }
}
