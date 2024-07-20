# IserverAccountOrdersGet200ResponseOrdersInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acct** | Option<**String**> | Account number | [optional]
**conidex** | Option<**String**> | conid and exchange. Format supports conid or conid@exchange | [optional]
**conid** | Option<**f64**> | Contract identifier | [optional]
**order_id** | Option<**String**> | Order identifier | [optional]
**cash_ccy** | Option<**String**> | Cash currency | [optional]
**size_and_fills** | Option<**String**> | Quantity outstanding and total quantity concatenated with forward slash separator | [optional]
**order_desc** | Option<**String**> | Order description | [optional]
**description1** | Option<**String**> | Formatted ticker description | [optional]
**ticker** | Option<**String**> | Underlying symbol | [optional]
**sec_type** | Option<**String**> | Asset class | [optional]
**listing_exchange** | Option<**String**> | Listing Exchange | [optional]
**remaining_quantity** | Option<**f64**> | Quantity remaining | [optional]
**filled_quantity** | Option<**f64**> | Quantity filled | [optional]
**company_name** | Option<**String**> | Company Name | [optional]
**status** | Option<**String**> | Status of the order | [optional]
**orig_order_type** | Option<**String**> | Original order type | [optional]
**supports_tax_opt** | Option<**f64**> | Supports Tax Optimization with 0 for no and 1 for yes | [optional]
**last_execution_time** | Option<**f64**> | Last status update in format YYMMDDhhmms based in GMT | [optional]
**last_execution_time_r** | Option<**f64**> | Last status update unix time in ms | [optional]
**order_type** | Option<**String**> | Order type | [optional]
**order_ref** | Option<**String**> | Order reference | [optional]
**side** | Option<**String**> | The side of the market of the order.  * BUY: Buy contract near posted ask price  * SELL: Sell contract near posted bid price  * ASSN: Option Assignment, if BUYSELL=BUY and OptionType=PUT or BUYSELL=SELL and OptionType=CALL  * EXER: Option Exercise, if BUYSELL=SELL and OptionType=PUT or BUYSELL=BUY and OptionType=CALL  | [optional]
**time_in_force** | Option<**String**> | Time in force | [optional]
**price** | Option<**f64**> | Price of order | [optional]
**bg_color** | Option<**String**> | Background color in hex format | [optional]
**fg_color** | Option<**String**> | Foreground color in hex format | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


