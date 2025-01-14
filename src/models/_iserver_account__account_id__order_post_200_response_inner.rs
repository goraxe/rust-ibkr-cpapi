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
pub struct IserverAccountAccountIdOrderPost200ResponseInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Please note here, if the message is a question, you have to reply to question in order to submit the order successfully. See more in the \"/iserver/reply/{replyid}\" endpoint. 
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Vec<String>>,
}

impl IserverAccountAccountIdOrderPost200ResponseInner {
    pub fn new() -> IserverAccountAccountIdOrderPost200ResponseInner {
        IserverAccountAccountIdOrderPost200ResponseInner {
            id: None,
            message: None,
        }
    }
}

