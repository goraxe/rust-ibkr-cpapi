/*
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`iserver_account_account_id_order_order_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverAccountAccountIdOrderOrderIdDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`iserver_account_account_id_order_order_id_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverAccountAccountIdOrderOrderIdPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`iserver_account_account_id_order_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverAccountAccountIdOrderPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`iserver_account_account_id_order_whatif_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverAccountAccountIdOrderWhatifPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`iserver_account_account_id_orders_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverAccountAccountIdOrdersPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`iserver_account_account_id_orders_whatif_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverAccountAccountIdOrdersWhatifPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`iserver_account_order_status_order_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverAccountOrderStatusOrderIdGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`iserver_account_orders_fa_group_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverAccountOrdersFaGroupPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`iserver_account_orders_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverAccountOrdersGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`iserver_reply_replyid_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverReplyReplyidPostError {
    Status400(models::IserverReplyReplyidPost400Response),
    UnknownValue(serde_json::Value),
}


/// Cancels an open order. Must call /iserver/accounts endpoint prior to cancelling an order. Use /iservers/account/orders endpoint to review open-order(s) and get latest order status.
pub async fn iserver_account_account_id_order_order_id_delete(configuration: &configuration::Configuration, account_id: &str, order_id: &str) -> Result<models::IserverAccountAccountIdOrderOrderIdDelete200Response, Error<IserverAccountAccountIdOrderOrderIdDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iserver/account/{accountId}/order/{orderId}", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id), orderId=crate::apis::urlencode(order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverAccountAccountIdOrderOrderIdDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Modifies an open order. Must call /iserver/accounts endpoint prior to modifying an order. Use /iservers/account/orders endpoint to review open-order(s).
pub async fn iserver_account_account_id_order_order_id_post(configuration: &configuration::Configuration, account_id: &str, order_id: &str, body: models::ModifyOrder) -> Result<Vec<models::IserverAccountAccountIdOrderOrderIdPost200ResponseInner>, Error<IserverAccountAccountIdOrderOrderIdPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iserver/account/{accountId}/order/{orderId}", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id), orderId=crate::apis::urlencode(order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverAccountAccountIdOrderOrderIdPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint is going to be deprecated, you can use /iserver/account/{accountId}/orders, just pass one order in the array, the order structure will be same. Please note here, sometimes this endpoint alone can't make sure you submit the order successfully, you could receive some questions in the response, you have to to answer them in order to submit the order successfully. You can use \"/iserver/reply/{replyid}\" endpoint to answer questions 
pub async fn iserver_account_account_id_order_post(configuration: &configuration::Configuration, account_id: &str, body: models::OrderRequest) -> Result<Vec<models::IserverAccountAccountIdOrderPost200ResponseInner>, Error<IserverAccountAccountIdOrderPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iserver/account/{accountId}/order", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverAccountAccountIdOrderPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This end-point is going to be deprecated, you can use /iserver/account/{accountId}/orders/whatif, just pass one order in the array, the order structure will be same. This endpoint allows you to preview order without actually submitting the order and you can get commission information in the response. 
pub async fn iserver_account_account_id_order_whatif_post(configuration: &configuration::Configuration, account_id: &str, body: models::OrderRequest) -> Result<models::IserverAccountAccountIdOrderWhatifPost200Response, Error<IserverAccountAccountIdOrderWhatifPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iserver/account/{accountId}/order/whatif", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverAccountAccountIdOrderWhatifPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// When connected to an IServer Brokerage Session, this endpoint will allow you to submit orders.  CP WEB API supports various advanced orderTypes, for additional details and examples refer to [IBKR Quant Blog](https://www.tradersinsight.news/category/ibkr-quant-news/programming_languages/rest-development/).   * Bracket - Attach additional opposite-side order(s) by using a single **cOID** sent with the parent and set the same value for **parentId** in each child order(s).   * Cash Quantity -  Send orders using monetary value by specifying **cashQty** instead of quantity, e.g. cashQty: 200. The endpoint /iserver/contract/rules returns list of valid orderTypes in cqtTypes.   * Currency Conversion - Convert cash from one currency to another by including **isCcyConv** = **true**. To specify the cash quantity use **fxQTY** instead of quantity, e.g. fxQTY: 100.   * Fractional - Contracts that support fractional shares can be traded by specifying **quantity** as a float, e.g. quantity: 0.001. The endpoint /iserver/contract/rules returns a list of valid orderTypes in fraqTypes.   * IB Algos - Attached user-defined settings to your trades by using any of IBKR's Algo Orders. Use the endpoint /iserver/contract/{conid}/algos to identify the available strategies for a contract.   * One-Cancels-All (OCA) - Group multiple unrelated orders by passing order request info in an array and including **isSingleGroup = true** for each order. All orders will be assigned the same oca_group_id. 
pub async fn iserver_account_account_id_orders_post(configuration: &configuration::Configuration, account_id: &str, body: models::IserverAccountAccountIdOrdersPostRequest) -> Result<Vec<models::IserverAccountAccountIdOrderPost200ResponseInner>, Error<IserverAccountAccountIdOrdersPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iserver/account/{accountId}/orders", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverAccountAccountIdOrdersPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint allows you to preview order without actually submitting the order and you can get commission information in the response. Also supports bracket orders. 
pub async fn iserver_account_account_id_orders_whatif_post(configuration: &configuration::Configuration, account_id: &str, body: models::IserverAccountAccountIdOrdersPostRequest) -> Result<models::IserverAccountAccountIdOrderWhatifPost200Response, Error<IserverAccountAccountIdOrdersWhatifPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iserver/account/{accountId}/orders/whatif", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverAccountAccountIdOrdersWhatifPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn iserver_account_order_status_order_id_get(configuration: &configuration::Configuration, order_id: &str) -> Result<models::OrderStatus, Error<IserverAccountOrderStatusOrderIdGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iserver/account/order/status/{orderId}", local_var_configuration.base_path, orderId=crate::apis::urlencode(order_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverAccountOrderStatusOrderIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Financial Advisors can use this endpoint to place an order for a specified group. To place an order for a specified account use the endpoint /iserver/account/{accountId}/order. More information about groups can be found in the [TWS Users' Guide:](https://guides.interactivebrokers.com/twsguide/twsguide.htm#usersguidebook/financialadvisors/create_an_account_group_for_share_allocation.htm). 
pub async fn iserver_account_orders_fa_group_post(configuration: &configuration::Configuration, fa_group: &str, body: models::OrderRequest) -> Result<Vec<models::IserverAccountAccountIdOrderPost200ResponseInner>, Error<IserverAccountOrdersFaGroupPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iserver/account/orders/{faGroup}", local_var_configuration.base_path, faGroup=crate::apis::urlencode(fa_group));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverAccountOrdersFaGroupPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The endpoint is meant to be used in polling mode, e.g. requesting every x seconds. The response will contain two objects, one is notification, the other is orders. Orders is the list of live orders (cancelled, filled, submitted). Notifications contains information about execute orders as they happen, see status field. To receive streaming live orders the endpoint /ws can be used. Refer to [Streaming WebSocket Data](https://interactivebrokers.github.io/cpwebapi/RealtimeSubscription.html) for details. 
pub async fn iserver_account_orders_get(configuration: &configuration::Configuration, filters: Option<&str>) -> Result<models::IserverAccountOrdersGet200Response, Error<IserverAccountOrdersGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iserver/account/orders", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filters {
        local_var_req_builder = local_var_req_builder.query(&[("Filters", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverAccountOrdersGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Reply to questions when placing orders and submit orders
pub async fn iserver_reply_replyid_post(configuration: &configuration::Configuration, replyid: &str, body: models::IserverReplyReplyidPostRequest) -> Result<Vec<models::IserverReplyReplyidPost200ResponseInner>, Error<IserverReplyReplyidPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/iserver/reply/{replyid}", local_var_configuration.base_path, replyid=crate::apis::urlencode(replyid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverReplyReplyidPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

