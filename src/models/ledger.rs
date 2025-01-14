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
pub struct Ledger {
    #[serde(rename = "commoditymarketvalue", skip_serializing_if = "Option::is_none")]
    pub commoditymarketvalue: Option<f64>,
    #[serde(rename = "futuremarketvalue", skip_serializing_if = "Option::is_none")]
    pub futuremarketvalue: Option<f64>,
    #[serde(rename = "settledcash", skip_serializing_if = "Option::is_none")]
    pub settledcash: Option<f64>,
    #[serde(rename = "exchangerate", skip_serializing_if = "Option::is_none")]
    pub exchangerate: Option<f64>,
    #[serde(rename = "sessionid", skip_serializing_if = "Option::is_none")]
    pub sessionid: Option<i32>,
    #[serde(rename = "cashbalance", skip_serializing_if = "Option::is_none")]
    pub cashbalance: Option<f64>,
    #[serde(rename = "corporatebondsmarketvalue", skip_serializing_if = "Option::is_none")]
    pub corporatebondsmarketvalue: Option<f64>,
    #[serde(rename = "warrantsmarketvalue", skip_serializing_if = "Option::is_none")]
    pub warrantsmarketvalue: Option<f64>,
    #[serde(rename = "netliquidationvalue", skip_serializing_if = "Option::is_none")]
    pub netliquidationvalue: Option<f64>,
    #[serde(rename = "interest", skip_serializing_if = "Option::is_none")]
    pub interest: Option<f64>,
    #[serde(rename = "unrealizedpnl", skip_serializing_if = "Option::is_none")]
    pub unrealizedpnl: Option<f64>,
    #[serde(rename = "stockmarketvalue", skip_serializing_if = "Option::is_none")]
    pub stockmarketvalue: Option<f64>,
    #[serde(rename = "moneyfunds", skip_serializing_if = "Option::is_none")]
    pub moneyfunds: Option<f64>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "realizedpnl", skip_serializing_if = "Option::is_none")]
    pub realizedpnl: Option<f64>,
    #[serde(rename = "funds", skip_serializing_if = "Option::is_none")]
    pub funds: Option<f64>,
    #[serde(rename = "acctcode", skip_serializing_if = "Option::is_none")]
    pub acctcode: Option<String>,
    #[serde(rename = "issueroptionsmarketvalue", skip_serializing_if = "Option::is_none")]
    pub issueroptionsmarketvalue: Option<f64>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger {
            commoditymarketvalue: None,
            futuremarketvalue: None,
            settledcash: None,
            exchangerate: None,
            sessionid: None,
            cashbalance: None,
            corporatebondsmarketvalue: None,
            warrantsmarketvalue: None,
            netliquidationvalue: None,
            interest: None,
            unrealizedpnl: None,
            stockmarketvalue: None,
            moneyfunds: None,
            currency: None,
            realizedpnl: None,
            funds: None,
            acctcode: None,
            issueroptionsmarketvalue: None,
            key: None,
            timestamp: None,
            severity: None,
        }
    }
}

