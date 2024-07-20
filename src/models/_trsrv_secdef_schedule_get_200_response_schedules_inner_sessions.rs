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

/// TrsrvSecdefScheduleGet200ResponseSchedulesInnerSessions : If the LIQUID hours differs from the total trading day then a separate 'session' tag is returned.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrsrvSecdefScheduleGet200ResponseSchedulesInnerSessions {
    #[serde(rename = "openingTime", skip_serializing_if = "Option::is_none")]
    pub opening_time: Option<i32>,
    #[serde(rename = "closingTime", skip_serializing_if = "Option::is_none")]
    pub closing_time: Option<i32>,
    /// If the whole trading day is considered LIQUID then the value 'LIQUID' is returned.
    #[serde(rename = "prop", skip_serializing_if = "Option::is_none")]
    pub prop: Option<String>,
}

impl TrsrvSecdefScheduleGet200ResponseSchedulesInnerSessions {
    /// If the LIQUID hours differs from the total trading day then a separate 'session' tag is returned.
    pub fn new() -> TrsrvSecdefScheduleGet200ResponseSchedulesInnerSessions {
        TrsrvSecdefScheduleGet200ResponseSchedulesInnerSessions {
            opening_time: None,
            closing_time: None,
            prop: None,
        }
    }
}

