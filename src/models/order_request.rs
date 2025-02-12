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
pub struct OrderRequest {
    /// acctId is optional. It should be one of the accounts returned by /iserver/accounts. If not passed, the first one in the list is selected. 
    #[serde(rename = "acctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<String>,
    /// conid is the identifier of the security you want to trade, you can find the conid with /iserver/secdef/search. 
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i32>,
    /// Conid and Exchange - Can be used instead of conid when specifying the contract identifier of a security. 
    #[serde(rename = "conidex", skip_serializing_if = "Option::is_none")]
    pub conidex: Option<String>,
    /// The contract-identifier (conid) and security type (type) specified as a concatenated value, conid:type
    #[serde(rename = "secType", skip_serializing_if = "Option::is_none")]
    pub sec_type: Option<String>,
    /// Customer Order ID. An arbitrary string that can be used to identify the order, e.g \"my-fb-order\". The value must be unique for a 24h span. Please do not set this value for child orders when placing a bracket order. 
    #[serde(rename = "cOID", skip_serializing_if = "Option::is_none")]
    pub c_oid: Option<String>,
    /// Only specify for child orders when placing bracket orders. The parentId for the child order(s) must be equal to the cOId (customer order id) of the parent. 
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// The order-type determines what type of order you want to send.   * LMT - A limit order is an order to buy or sell at the specified price or better.   * MKT - A market order is an order to buy or sell at the markets current NBBO.   * STP - A stop order becomes a market order once the specified stop price is attained or penetrated.   * STOP_LIMIT - A stop limit order becomes a limit order once the specified stop price is attained or penetrated.   * MIDPRICE - A MidPrice order attempts to fill at the current midpoint of the NBBO or better.   * TRAIL - A sell trailing stop order sets the stop price at a fixed amount below the market price with an attached \"trailing\" amount. See more details here: https://ndcdyn.interactivebrokers.com/en/index.php?f=605   * TRAILLMT - A trailing stop limit order is designed to allow an investor to specify a limit on the maximum possible loss, without setting a limit on the maximum possible gain.     See more details here: https://ndcdyn.interactivebrokers.com/en/index.php?f=606 
    #[serde(rename = "orderType", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// listingExchange is optional. By default we use \"SMART\" routing. Possible values are available via the endpoint: /iserver/contract/{conid}/info, see **valid_exchange** e.g: SMART,AMEX,NYSE,CBOE,ISE,CHX,ARCA,ISLAND,DRCTEDGE,BEX,BATS,EDGEA,CSFBALGO,JE FFALGO,BYX,IEX,FOXRIVER,TPLUS1,NYSENAT,PSX 
    #[serde(rename = "listingExchange", skip_serializing_if = "Option::is_none")]
    pub listing_exchange: Option<String>,
    /// set to true if you want to place a single group orders(OCA) 
    #[serde(rename = "isSingleGroup", skip_serializing_if = "Option::is_none")]
    pub is_single_group: Option<bool>,
    /// set to true if the order can be executed outside regular trading hours. 
    #[serde(rename = "outsideRTH", skip_serializing_if = "Option::is_none")]
    pub outside_rth: Option<bool>,
    /// optional if order is LMT, or STOP_LIMIT, this is the limit price. For STP|TRAIL this is the stop price. For MIDPRICE this is the option price cap. 
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// optional if order is STOP_LIMIT|TRAILLMT, this is the stop price. You must specify both price and auxPrice for STOP_LIMIT|TRAILLMT orders. 
    #[serde(rename = "auxPrice", skip_serializing_if = "Option::is_none")]
    pub aux_price: Option<serde_json::Value>,
    /// SELL or BUY
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// This is the  underlying symbol for the contract. 
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// The Time-In-Force determines how long the order remains active on the market.   * GTC - use Good-Till-Cancel for orders to remain active until it executes or cancelled.   * OPG - use Open-Price-Guarantee for Limit-On-Open (LOO) or Market-On-Open (MOO) orders.   * DAY - if not executed a Day order will automatically cancel at the end of the markets regular trading hours.   * IOC - any portion of an Immediate-or-Cancel order that is not filled as soon as it becomes available in the market is cancelled. 
    #[serde(rename = "tif", skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
    /// optional if order is TRAIL, or TRAILLMT. When trailingType is amt, this is the trailing amount, when trailingType is %, it means percentage. You must specify both trailingType and trailingAmt for TRAIL and TRAILLMT order 
    #[serde(rename = "trailingAmt", skip_serializing_if = "Option::is_none")]
    pub trailing_amt: Option<f64>,
    /// This is the trailing type for trailing amount. We only support two types here: amt or %. You must specify both trailingType and trailingAmt for TRAIL and TRAILLMT order 
    #[serde(rename = "trailingType", skip_serializing_if = "Option::is_none")]
    pub trailing_type: Option<String>,
    /// Custom order reference 
    #[serde(rename = "referrer", skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    /// Usually integer, for some special cases such as fractional orders can specify as a float, e.g. quantity = 0.001. In some special cases quantity is not specified, such as when using 'cashQty' or 'fxQty'. 
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    /// Cash Quantity - used to specify the monetary value of an order instead of the number of shares. When using 'cashQty' don't specify 'quantity' Orders that express size using a monetary value, e.g. cash quantity can result in fractional shares and are provided on a non-guaranteed basis. The system simulates the order by canceling it once the specified amount is spent (for buy orders) or collected (for sell orders). In addition to the monetary value, the order uses a maximum size that is calculated using the Cash Quantity Estimated Factor, which can be modified in Order Presets.   
    #[serde(rename = "cashQty", skip_serializing_if = "Option::is_none")]
    pub cash_qty: Option<f64>,
    /// double number, this is the cash quantity field which can only be used for Currency Conversion Orders. When using 'fxQty' don't specify 'quantity'. 
    #[serde(rename = "fxQty", skip_serializing_if = "Option::is_none")]
    pub fx_qty: Option<f64>,
    /// If true, the system will use the Price Management Algo to submit the order. https://www.interactivebrokers.com/en/index.php?f=43423 
    #[serde(rename = "useAdaptive", skip_serializing_if = "Option::is_none")]
    pub use_adaptive: Option<bool>,
    /// set to true if the order is a FX conversion order 
    #[serde(rename = "isCcyConv", skip_serializing_if = "Option::is_none")]
    pub is_ccy_conv: Option<bool>,
    /// Set the allocation method when placing an order using an FA account for a group Possible allocation methods are \"NetLiquidity\", \"AvailableEquity\", \"EqualQuantity\" and \"PctChange\". 
    #[serde(rename = "allocationMethod", skip_serializing_if = "Option::is_none")]
    pub allocation_method: Option<String>,
    /// Specify which IB Algo algorithm to use for this order. 
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    /// The IB Algo parameters for the specified algorithm. 
    #[serde(rename = "strategyParameters", skip_serializing_if = "Option::is_none")]
    pub strategy_parameters: Option<serde_json::Value>,
}

impl OrderRequest {
    pub fn new() -> OrderRequest {
        OrderRequest {
            acct_id: None,
            conid: None,
            conidex: None,
            sec_type: None,
            c_oid: None,
            parent_id: None,
            order_type: None,
            listing_exchange: None,
            is_single_group: None,
            outside_rth: None,
            price: None,
            aux_price: None,
            side: None,
            ticker: None,
            tif: None,
            trailing_amt: None,
            trailing_type: None,
            referrer: None,
            quantity: None,
            cash_qty: None,
            fx_qty: None,
            use_adaptive: None,
            is_ccy_conv: None,
            allocation_method: None,
            strategy: None,
            strategy_parameters: None,
        }
    }
}

