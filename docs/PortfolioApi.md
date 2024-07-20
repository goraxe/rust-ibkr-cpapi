# \PortfolioApi

All URIs are relative to *http://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**portfolio_account_id_allocation_get**](PortfolioApi.md#portfolio_account_id_allocation_get) | **GET** /portfolio/{accountId}/allocation | Account Allocation
[**portfolio_account_id_ledger_get**](PortfolioApi.md#portfolio_account_id_ledger_get) | **GET** /portfolio/{accountId}/ledger | Account Ledger
[**portfolio_account_id_meta_get**](PortfolioApi.md#portfolio_account_id_meta_get) | **GET** /portfolio/{accountId}/meta | Account Information
[**portfolio_account_id_position_conid_get**](PortfolioApi.md#portfolio_account_id_position_conid_get) | **GET** /portfolio/{accountId}/position/{conid} | Position by Conid
[**portfolio_account_id_positions_invalidate_post**](PortfolioApi.md#portfolio_account_id_positions_invalidate_post) | **POST** /portfolio/{accountId}/positions/invalidate | Invalidates the backend cache of the Portfolio
[**portfolio_account_id_positions_page_id_get**](PortfolioApi.md#portfolio_account_id_positions_page_id_get) | **GET** /portfolio/{accountId}/positions/{pageId} | Portfolio Positions
[**portfolio_account_id_summary_get**](PortfolioApi.md#portfolio_account_id_summary_get) | **GET** /portfolio/{accountId}/summary | Account Summary
[**portfolio_accounts_get**](PortfolioApi.md#portfolio_accounts_get) | **GET** /portfolio/accounts | Portfolio Accounts
[**portfolio_allocation_post**](PortfolioApi.md#portfolio_allocation_post) | **POST** /portfolio/allocation | Account Alloction (All Accounts)
[**portfolio_positions_conid_get**](PortfolioApi.md#portfolio_positions_conid_get) | **GET** /portfolio/positions/{conid} | Positions by Conid
[**portfolio_subaccounts2_page_get**](PortfolioApi.md#portfolio_subaccounts2_page_get) | **GET** /portfolio/subaccounts2/{page} | List of Sub-Accounts (Large Accounts)
[**portfolio_subaccounts_get**](PortfolioApi.md#portfolio_subaccounts_get) | **GET** /portfolio/subaccounts | List of Sub-Accounts



## portfolio_account_id_allocation_get

> Vec<models::AllocationInner> portfolio_account_id_allocation_get(account_id)
Account Allocation

Information about the account's portfolio allocation by Asset Class, Industry and Category.  /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |

### Return type

[**Vec<models::AllocationInner>**](allocation_inner.md)

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


## portfolio_account_id_position_conid_get

> Vec<models::PositionInner> portfolio_account_id_position_conid_get(account_id, conid)
Position by Conid

Returns a list of all positions matching the conid. For portfolio models the conid could be in more than one model, returning an array with the name of the model it belongs to.  /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**conid** | **i32** | contract id | [required] |

### Return type

[**Vec<models::PositionInner>**](position_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_positions_invalidate_post

> serde_json::Value portfolio_account_id_positions_invalidate_post(account_id)
Invalidates the backend cache of the Portfolio

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_positions_page_id_get

> Vec<models::PositionInner> portfolio_account_id_positions_page_id_get(account_id, page_id, model, sort, direction, period)
Portfolio Positions

Returns a list of positions for the given account. The endpoint supports paging, page's default size is 30 positions. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**page_id** | **String** | page id | [required] |[default to 0]
**model** | Option<**String**> | optional |  |
**sort** | Option<**String**> | declare the table to be sorted by which column |  |
**direction** | Option<**String**> | in which order, a means ascending - d means descending |  |
**period** | Option<**String**> | period for pnl column, can be 1D, 7D, 1M... |  |

### Return type

[**Vec<models::PositionInner>**](position_inner.md)

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


## portfolio_allocation_post

> Vec<models::AllocationInner> portfolio_allocation_post(body)
Account Alloction (All Accounts)

Similar to /portfolio/{accountId}/allocation but returns a consolidated view of of all the accounts returned by /portfolio/accounts. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PaSummaryPostRequest**](PaSummaryPostRequest.md) | accounts info | [required] |

### Return type

[**Vec<models::AllocationInner>**](allocation_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_positions_conid_get

> models::PortfolioPositionsConidGet200Response portfolio_positions_conid_get(conid)
Positions by Conid

Returns an object of all positions matching the conid for all the selected accounts. For portfolio models the conid could be in more than one model, returning an array with the name of the model it belongs to. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **i32** | contract id | [required] |

### Return type

[**models::PortfolioPositionsConidGet200Response**](_portfolio_positions__conid__get_200_response.md)

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

