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
pub struct IserverContractRulesPost200ResponseRulesInnerOrderDefaultsInnerStringInner {
    /// Outside of Regular Trading Hours
    #[serde(rename = "ORTH", skip_serializing_if = "Option::is_none")]
    pub orth: Option<bool>,
    /// Stop Price value
    #[serde(rename = "SP", skip_serializing_if = "Option::is_none")]
    pub sp: Option<String>,
    /// Limit Price value
    #[serde(rename = "LP", skip_serializing_if = "Option::is_none")]
    pub lp: Option<String>,
    /// Price Cap value
    #[serde(rename = "PC", skip_serializing_if = "Option::is_none")]
    pub pc: Option<String>,
    /// Trailing amount value
    #[serde(rename = "TA", skip_serializing_if = "Option::is_none")]
    pub ta: Option<String>,
    /// Trailing unit
    #[serde(rename = "TU", skip_serializing_if = "Option::is_none")]
    pub tu: Option<String>,
    /// Releative offset amount
    #[serde(rename = "ROA", skip_serializing_if = "Option::is_none")]
    pub roa: Option<String>,
    /// Relative offset percent
    #[serde(rename = "ROP", skip_serializing_if = "Option::is_none")]
    pub rop: Option<String>,
    /// Touch trigger price
    #[serde(rename = "TT", skip_serializing_if = "Option::is_none")]
    pub tt: Option<String>,
    /// Use Net Price for Bonds
    #[serde(rename = "UNP", skip_serializing_if = "Option::is_none")]
    pub unp: Option<bool>,
}

impl IserverContractRulesPost200ResponseRulesInnerOrderDefaultsInnerStringInner {
    pub fn new() -> IserverContractRulesPost200ResponseRulesInnerOrderDefaultsInnerStringInner {
        IserverContractRulesPost200ResponseRulesInnerOrderDefaultsInnerStringInner {
            orth: None,
            sp: None,
            lp: None,
            pc: None,
            ta: None,
            tu: None,
            roa: None,
            rop: None,
            tt: None,
            unp: None,
        }
    }
}

