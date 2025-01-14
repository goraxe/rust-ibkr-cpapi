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

/// TrsrvSecdefScheduleGet200ResponseSchedulesInnerTradingTimes : Returns tradingTime in exchange time zone.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrsrvSecdefScheduleGet200ResponseSchedulesInnerTradingTimes {
    #[serde(rename = "openingTime", skip_serializing_if = "Option::is_none")]
    pub opening_time: Option<i32>,
    #[serde(rename = "closingTime", skip_serializing_if = "Option::is_none")]
    pub closing_time: Option<i32>,
    #[serde(rename = "cancelDayOrders", skip_serializing_if = "Option::is_none")]
    pub cancel_day_orders: Option<String>,
}

impl TrsrvSecdefScheduleGet200ResponseSchedulesInnerTradingTimes {
    /// Returns tradingTime in exchange time zone.
    pub fn new() -> TrsrvSecdefScheduleGet200ResponseSchedulesInnerTradingTimes {
        TrsrvSecdefScheduleGet200ResponseSchedulesInnerTradingTimes {
            opening_time: None,
            closing_time: None,
            cancel_day_orders: None,
        }
    }
}

