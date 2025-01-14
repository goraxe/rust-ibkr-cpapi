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

/// SecdefInfo : Contains some basic info of contract
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecdefInfo {
    /// IBKR contract identifier
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<f64>,
    /// Underlying symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Security type
    #[serde(rename = "secType", skip_serializing_if = "Option::is_none")]
    pub sec_type: Option<String>,
    /// Primary Exchange, Routing or Trading Venue
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Main Trading Venue
    #[serde(rename = "listingExchange", skip_serializing_if = "Option::is_none")]
    pub listing_exchange: Option<String>,
    /// Put or Call of the option. C = Call Option, P = Put Option
    #[serde(rename = "right", skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
    /// Set price at which a derivative contract can be bought or sold. The strike price also known as exercise price.
    #[serde(rename = "strike", skip_serializing_if = "Option::is_none")]
    pub strike: Option<f64>,
    /// Currency the contract trades in
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Committee on Uniform Securities Identification Procedures number
    #[serde(rename = "cusip", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    /// Annual interest rate paid on a bond
    #[serde(rename = "coupon", skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// Currency pairs for Forex e.g. EUR.AUD, EUR.CAD, EUR.CHF etc.
    #[serde(rename = "desc1", skip_serializing_if = "Option::is_none")]
    pub desc1: Option<String>,
    /// Formatted expiration, strike and right
    #[serde(rename = "desc2", skip_serializing_if = "Option::is_none")]
    pub desc2: Option<String>,
    /// Format YYYYMMDD, the date on which the underlying transaction settles if the option is exercised
    #[serde(rename = "maturityDate", skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<f64>,
    /// Multiplier for total premium paid or received for derivative contract.
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<String>,
    /// Designation of the contract.
    #[serde(rename = "tradingClass", skip_serializing_if = "Option::is_none")]
    pub trading_class: Option<String>,
    /// Comma separated list of exchanges or trading venues.
    #[serde(rename = "validExchanges", skip_serializing_if = "Option::is_none")]
    pub valid_exchanges: Option<String>,
}

impl SecdefInfo {
    /// Contains some basic info of contract
    pub fn new() -> SecdefInfo {
        SecdefInfo {
            conid: None,
            symbol: None,
            sec_type: None,
            exchange: None,
            listing_exchange: None,
            right: None,
            strike: None,
            currency: None,
            cusip: None,
            coupon: None,
            desc1: None,
            desc2: None,
            maturity_date: None,
            multiplier: None,
            trading_class: None,
            valid_exchanges: None,
        }
    }
}

