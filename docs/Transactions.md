# Transactions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | will always be getTransactions | [optional]
**currency** | Option<**String**> | same as request | [optional]
**includes_real_time** | Option<**bool**> | Indicates whether current day and realtime data is included in the result | [optional]
**from** | Option<**f64**> | Period start date. Epoch time, GMT | [optional]
**to** | Option<**f64**> | Period end date. Epoch time, GMT | [optional]
**transactions** | Option<[**Vec<models::TransactionsTransactionsInner>**](transactions_transactions_inner.md)> | Sorted by date descending | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


