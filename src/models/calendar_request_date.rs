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
pub struct CalendarRequestDate {
    /// start date of a period. for example 20180808-0400
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// end date of a period. for example 20180808-0400
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

impl CalendarRequestDate {
    pub fn new() -> CalendarRequestDate {
        CalendarRequestDate {
            start: None,
            end: None,
        }
    }
}
