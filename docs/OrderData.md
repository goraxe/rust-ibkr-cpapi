# OrderData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_order_id** | Option<**String**> |  | [optional]
**exec_id** | Option<**String**> |  | [optional]
**exec_type** | Option<**String**> |  | [optional]
**order_type** | Option<**String**> |  | [optional]
**order_status** | Option<**String**> |  | [optional]
**symbol** | Option<**String**> | Underlying symbol for contract | [optional]
**order_qty** | Option<**String**> | Quantity of active order | [optional]
**price** | Option<**String**> | Price of active order | [optional]
**last_shares** | Option<**String**> | Quantity of the last partial fill | [optional]
**last_price** | Option<**String**> | Price of the last partial fill | [optional]
**cum_qty** | Option<**String**> | Cumulative fill quantity | [optional]
**leaves_qty** | Option<**String**> | Remaining quantity to be filled | [optional]
**avg_price** | Option<**String**> | Average fill price | [optional]
**side** | Option<**String**> |  | [optional]
**order_id** | Option<**String**> | Order identifier | [optional]
**account** | Option<**String**> | Account number | [optional]
**sec_type** | Option<**String**> | Contracts asset class | [optional]
**tx_time** | Option<**String**> | Time of transaction in GMT, format YYYYMMDD-hh:m:ss | [optional]
**rcpt_time** | Option<**String**> | Time of receipt in GMT, format YYYYMMDD-hh:mm:ss | [optional]
**tif** | Option<**String**> | Time in Force | [optional]
**conid** | Option<**String**> | Contract identifier from IBKR's database. | [optional]
**currency** | Option<**String**> | Trading currency | [optional]
**exchange** | Option<**String**> | Exchange or venue | [optional]
**listing_exchange** | Option<**String**> | Listing Exchange | [optional]
**text** | Option<**f64**> | error message | [optional]
**warnings** | Option<[**models::OrderDataWarnings**](order_data_warnings.md)> |  | [optional]
**comm_curr** | Option<**String**> | Commission currency | [optional]
**comms** | Option<**String**> | Commissions | [optional]
**realized_pnl** | Option<**String**> | Realized PnL | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


