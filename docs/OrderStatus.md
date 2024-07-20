# OrderStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sub_type** | Option<**String**> | order sub-type | [optional]
**request_id** | Option<**String**> | order request id | [optional]
**order_id** | Option<**i32**> | system generated order id, unique per account | [optional]
**conidex** | Option<**String**> | conid and exchange. Format supports conid or conid@exchange | [optional]
**symbol** | Option<**String**> | Underlying symbol | [optional]
**side** | Option<**String**> | The side of the market of the order.   * B - Buy contract near posted ask price   * S - Sell contract near posted bid price   * X - Option expired  | [optional]
**contract_description_1** | Option<**String**> | Format contract name | [optional]
**listing_exchange** | Option<**String**> | Trading Exchange or Venue | [optional]
**option_acct** | Option<**String**> |  | [optional]
**company_name** | Option<**String**> | Contracts company name | [optional]
**size** | Option<**String**> | Quantity updated | [optional]
**total_size** | Option<**String**> | Total quantity | [optional]
**currency** | Option<**String**> | Contract traded currency | [optional]
**account** | Option<**String**> | account id | [optional]
**order_type** | Option<**String**> | Types of orders | [optional]
**limit_price** | Option<**String**> | Limit price | [optional]
**stop_price** | Option<**String**> | Stop price | [optional]
**cum_fill** | Option<**String**> | Cumulative fill | [optional]
**order_status** | Option<**String**> | *  PendingSubmit - Indicates the order was sent, but confirmation has not been received that it has been received by the destination.                    Occurs most commonly if an exchange is closed. *  PendingCancel - Indicates that a request has been sent to cancel an order but confirmation has not been received of its cancellation. *  PreSubmitted - Indicates that a simulated order type has been accepted by the IBKR system and that this order has yet to be elected.                   The order is held in the IBKR system until the election criteria are met. At that time the order is transmitted to the order destination as specified. *  Submitted - Indicates that the order has been accepted at the order destination and is working. *  Cancelled - Indicates that the balance of the order has been confirmed cancelled by the IB system.                This could occur unexpectedly when IB or the destination has rejected the order. *  Filled - Indicates that the order has been completely filled. *  Inactive - Indicates the order is not working, for instance if the order was invalid and triggered an error message,               or if the order was to short a security and shares have not yet been located.  | [optional]
**order_status_description** | Option<**String**> | Description of the order status | [optional]
**tif** | Option<**String**> | Time-in-Force - length of time order will continue working before it is canceled. | [optional]
**fg_color** | Option<**String**> | Foreground color in hex format | [optional]
**bg_color** | Option<**String**> | Background color in hex format | [optional]
**order_not_editable** | Option<**bool**> | If true not allowed to modify order | [optional]
**editable_fields** | Option<**String**> | Fields that can be edited in escaped unicode characters | [optional]
**cannot_cancel_order** | Option<**bool**> | If true not allowed to cancel order | [optional]
**outside_rth** | Option<**bool**> | If true order trades outside regular trading hours | [optional]
**deactivate_order** | Option<**bool**> | If true order is de-activated | [optional]
**use_price_mgmt_algo** | Option<**bool**> | If true price management algo is enabled, refer to https://www.interactivebrokers.com/en/index.php?f=43423 | [optional]
**sec_type** | Option<**String**> | Asset class | [optional]
**available_chart_periods** | Option<**String**> | List of available chart periods | [optional]
**order_description** | Option<**String**> | Format description of order | [optional]
**order_description_with_contract** | Option<**String**> | order_description with the symbol | [optional]
**alert_active** | Option<**i32**> |  | [optional]
**child_order_type** | Option<**String**> | type of the child order | [optional]
**size_and_fills** | Option<**String**> | Format fillQuantity\\totalQuantity | [optional]
**exit_strategy_display_price** | Option<**String**> | Position display price | [optional]
**exit_strategy_chart_description** | Option<**String**> | Position description to display on chart | [optional]
**exit_strategy_tool_availability** | Option<**String**> | * 1: If your account has position or order for contract * 0: If your account has no position or order for contract  | [optional]
**allowed_duplicate_opposite** | Option<**bool**> | Returns true if contract supports duplicate/opposite side order. | [optional]
**order_time** | Option<**String**> | Time of status update in unix time | [optional]
**oca_group_id** | Option<**String**> | only exists for oca orders, oca orders in same group will have same id | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


