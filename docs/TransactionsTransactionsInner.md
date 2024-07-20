# TransactionsTransactionsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acctid** | Option<**String**> |  | [optional]
**conid** | Option<**f64**> |  | [optional]
**cur** | Option<**String**> | currency code | [optional]
**fx_rate** | Option<**f64**> | Conversion rate from asset currency to response currency | [optional]
**desc** | Option<**String**> | Transaction description | [optional]
**date** | Option<**String**> | Date of transaction.  Epoch time, GMT | [optional]
**r#type** | Option<**String**> | Transaction Type Name: Examples: \"Sell\", \"Buy\", \"Corporate Action\", \"Dividend Payment\", \"Transfer\", \"Payment in Lieu\" Dividends and Transfers do not have price and quantity in response  | [optional]
**qty** | Option<**f64**> | Not applicable for all transaction types | [optional]
**pr** | Option<**f64**> | In asset currency. Not be applicable for all transaction types. | [optional]
**amt** | Option<**f64**> | Raw value, no formatting. Net transaction amount (may include commission, tax). In asset currency | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


