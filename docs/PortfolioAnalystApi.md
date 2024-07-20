# \PortfolioAnalystApi

All URIs are relative to *http://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pa_performance_post**](PortfolioAnalystApi.md#pa_performance_post) | **POST** /pa/performance | Account Performance
[**pa_summary_post**](PortfolioAnalystApi.md#pa_summary_post) | **POST** /pa/summary | Account Balance's Summary (Deprecated)
[**pa_transactions_post**](PortfolioAnalystApi.md#pa_transactions_post) | **POST** /pa/transactions | Position's Transaction History



## pa_performance_post

> models::Performance pa_performance_post(body)
Account Performance

Returns the performance (MTM) for the given accounts, if more than one account is passed, the result is consolidated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PaPerformancePostRequest**](PaPerformancePostRequest.md) | an array of account ids | [required] |

### Return type

[**models::Performance**](performance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pa_summary_post

> models::Summary pa_summary_post(body)
Account Balance's Summary (Deprecated)

This endpoint is going to be deprecated. Please use /pa/performance instead. Returns a summary of all account balances for the given accounts, if more than one account is passe, the result is consolidated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PaSummaryPostRequest**](PaSummaryPostRequest.md) | an array of account ids | [required] |

### Return type

[**models::Summary**](summary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pa_transactions_post

> models::Transactions pa_transactions_post(body)
Position's Transaction History

transaction history for a given number of conids and accounts. Types of transactions include dividend payments, buy and sell transactions, transfers. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PaTransactionsPostRequest**](PaTransactionsPostRequest.md) |  | [required] |

### Return type

[**models::Transactions**](transactions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

