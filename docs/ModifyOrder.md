# ModifyOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acct_id** | Option<**String**> |  | [optional]
**conid** | Option<**i32**> |  | [optional]
**order_type** | Option<**String**> | for example LMT | [optional]
**outside_rth** | Option<**bool**> |  | [optional]
**price** | Option<**f64**> |  | [optional]
**aux_price** | Option<**f64**> |  | [optional]
**side** | Option<**String**> | SELL or BUY | [optional]
**listing_exchange** | Option<**String**> | optional, not required | [optional]
**ticker** | Option<**String**> | The ticker symbol of the original place order | [optional]
**tif** | Option<**String**> | Specify a time in force to change how long your order will continue to work in the market | [optional]
**quantity** | Option<**f64**> | usually integer, for some special cases can be float numbers | [optional]
**deactivated** | Option<**bool**> | Set to true if you want to pause a working order. For details refer to the [TWS Users' Guide:](https://guides.interactivebrokers.com/tws/twsguide.html#usersguidebook/getstarted/pause_execution.htm)  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


