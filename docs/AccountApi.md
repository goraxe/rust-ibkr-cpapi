# \AccountApi

All URIs are relative to *http://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_pnl_partitioned_get**](AccountApi.md#iserver_account_pnl_partitioned_get) | **GET** /iserver/account/pnl/partitioned | PnL for the selected account
[**iserver_account_post**](AccountApi.md#iserver_account_post) | **POST** /iserver/account | Switch Account
[**iserver_accounts_get**](AccountApi.md#iserver_accounts_get) | **GET** /iserver/accounts | Brokerage Accounts
[**portfolio_account_id_ledger_get**](AccountApi.md#portfolio_account_id_ledger_get) | **GET** /portfolio/{accountId}/ledger | Account Ledger
[**portfolio_account_id_meta_get**](AccountApi.md#portfolio_account_id_meta_get) | **GET** /portfolio/{accountId}/meta | Account Information
[**portfolio_account_id_summary_get**](AccountApi.md#portfolio_account_id_summary_get) | **GET** /portfolio/{accountId}/summary | Account Summary
[**portfolio_accounts_get**](AccountApi.md#portfolio_accounts_get) | **GET** /portfolio/accounts | Portfolio Accounts
[**portfolio_subaccounts2_page_get**](AccountApi.md#portfolio_subaccounts2_page_get) | **GET** /portfolio/subaccounts2/{page} | List of Sub-Accounts (Large Accounts)
[**portfolio_subaccounts_get**](AccountApi.md#portfolio_subaccounts_get) | **GET** /portfolio/subaccounts | List of Sub-Accounts



## iserver_account_pnl_partitioned_get

> models::IserverAccountPnlPartitionedGet200Response iserver_account_pnl_partitioned_get()
PnL for the selected account

Returns an object containing PnL for the selected account and its models (if any). To receive streaming PnL the endpoint /ws can be used. Refer to [Streaming WebSocket Data](https://interactivebrokers.github.io/cpwebapi/RealtimeSubscription.html) for details. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IserverAccountPnlPartitionedGet200Response**](_iserver_account_pnl_partitioned_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_post

> models::IserverAccountPost200Response iserver_account_post(body)
Switch Account

If an user has multiple accounts, and user wants to get orders, trades, etc. of an account other than currently selected account, then user can update the currently selected account using this API and then can fetch required information for the newly updated account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SetAccount**](SetAccount.md) | account id | [required] |

### Return type

[**models::IserverAccountPost200Response**](_iserver_account_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_accounts_get

> models::IserverAccountsGet200Response iserver_accounts_get()
Brokerage Accounts

Returns a list of accounts the user has trading access to, their respective aliases and the currently selected account. Note this endpoint must be called before modifying an order or querying open orders.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IserverAccountsGet200Response**](_iserver_accounts_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_ledger_get

> models::PortfolioAccountIdLedgerGet200Response portfolio_account_id_ledger_get(account_id)
Account Ledger

Information regarding settled cash, cash balances, etc. in the account's base currency and any other cash balances hold in other currencies.  /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint. The list of supported currencies is available at https://www.interactivebrokers.com/en/index.php?f=3185.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |

### Return type

[**models::PortfolioAccountIdLedgerGet200Response**](_portfolio__accountId__ledger_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_meta_get

> Vec<models::Account> portfolio_account_id_meta_get(account_id)
Account Information

Account information related to account Id /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |

### Return type

[**Vec<models::Account>**](account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_summary_get

> models::PortfolioAccountIdSummaryGet200Response portfolio_account_id_summary_get(account_id)
Account Summary

Returns information about margin, cash balances and other information related to specified account. See also /portfolio/{accountId}/ledger. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |

### Return type

[**models::PortfolioAccountIdSummaryGet200Response**](_portfolio__accountId__summary_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_accounts_get

> Vec<models::Account> portfolio_accounts_get()
Portfolio Accounts

In non-tiered account structures, returns a list of accounts for which the user can view position and account information. This endpoint must be called prior to calling other /portfolio endpoints for those accounts. For querying a list of accounts which the user can trade, see /iserver/accounts. For a list of subaccounts in tiered account structures (e.g. financial advisor or ibroker accounts) see /portfolio/subaccounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Account>**](account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_subaccounts2_page_get

> models::PortfolioSubaccounts2PageGet200Response portfolio_subaccounts2_page_get(page)
List of Sub-Accounts (Large Accounts)

Used in tiered account structures (such as Financial Advisor and IBroker Accounts) to return a list of sub-accounts, paginated up to 20 accounts per page, for which the user can view position and account-related information. This endpoint must be called prior to calling other /portfolio endpoints for those sub-accounts. If you have less than 100 sub-accounts use /portfolio/subaccounts. To query a list of accounts the user can trade, see /iserver/accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **String** |  | [required] |[default to 0]

### Return type

[**models::PortfolioSubaccounts2PageGet200Response**](_portfolio_subaccounts2__page__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_subaccounts_get

> models::Account portfolio_subaccounts_get()
List of Sub-Accounts

Used in tiered account structures (such as Financial Advisor and IBroker Accounts) to return a list of up to 100 sub-accounts for which the user can view position and account-related information. This endpoint must be called prior to calling other /portfolio endpoints for those sub-accounts. If you have more than 100 sub-accounts use /portfolio/subaccounts2. To query a list of accounts the user can trade, see /iserver/accounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Account**](account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

