# \CcpBetaApi

All URIs are relative to *http://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ccp_account_get**](CcpBetaApi.md#ccp_account_get) | **GET** /ccp/account | Brokerage Accounts
[**ccp_auth_init_post**](CcpBetaApi.md#ccp_auth_init_post) | **POST** /ccp/auth/init | Start CCP Session
[**ccp_auth_response_post**](CcpBetaApi.md#ccp_auth_response_post) | **POST** /ccp/auth/response | Complete CCP Session
[**ccp_order_delete**](CcpBetaApi.md#ccp_order_delete) | **DELETE** /ccp/order | Delete Order
[**ccp_order_post**](CcpBetaApi.md#ccp_order_post) | **POST** /ccp/order | Submit Order
[**ccp_order_put**](CcpBetaApi.md#ccp_order_put) | **PUT** /ccp/order | Update Order
[**ccp_orders_get**](CcpBetaApi.md#ccp_orders_get) | **GET** /ccp/orders | Order Status
[**ccp_positions_get**](CcpBetaApi.md#ccp_positions_get) | **GET** /ccp/positions | Positions
[**ccp_status_get**](CcpBetaApi.md#ccp_status_get) | **GET** /ccp/status | CCP Status
[**ccp_trades_get**](CcpBetaApi.md#ccp_trades_get) | **GET** /ccp/trades | Trades



## ccp_account_get

> models::CcpAccountGet200Response ccp_account_get()
Brokerage Accounts

Provides the list of tradeable accounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CcpAccountGet200Response**](_ccp_account_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ccp_auth_init_post

> models::CcpAuthInitPost200Response ccp_auth_init_post(compete, locale, mac, machine_id, username)
Start CCP Session

Initiate a brokerage session to CCP. Only one brokerage session type can run at a time. If an existing brokerage session to iServer is running then call the endpoint /logout first. Note at this time only order management is possible from CCP session, market data and scanner endpoints can't be used since they are only available from iServer session. Work is in progress to provide new CCP endpoints for market data and scanners.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compete** | Option<**bool**> | Allow competing CCP session to run |  |
**locale** | Option<**String**> | Concatenate value for language and region, set to \\\"en_US\\\" |  |
**mac** | Option<**String**> | Local MAC Address |  |
**machine_id** | Option<**String**> | Local machine ID |  |
**username** | Option<**String**> | Login user, set to dash \\\"-\\\" |  |

### Return type

[**models::CcpAuthInitPost200Response**](_ccp_auth_init_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ccp_auth_response_post

> models::CcpAuthResponsePost200Response ccp_auth_response_post(auth)
Complete CCP Session

Session Token Authentication

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth** | Option<[**CcpAuthResponsePostRequest**](CcpAuthResponsePostRequest.md)> |  |  |

### Return type

[**models::CcpAuthResponsePost200Response**](_ccp_auth_response_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ccp_order_delete

> models::OrderData ccp_order_delete(acct, id)
Delete Order

Sends an Order cancellation request. The status of the order can be queried through /ccp/order. Passing arguments as GET is also supported (requires passing action=delete) (GET is meant for development only) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acct** | **String** | Account Number | [required] |
**id** | **f64** | Order Identifier of original submit order | [required] |

### Return type

[**models::OrderData**](order-data.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ccp_order_post

> models::OrderData ccp_order_post(acct, conid, ccy, exchange, qty, r#type, side, price, tif)
Submit Order

Submits an Order. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acct** | **String** | User Account | [required] |
**conid** | **f64** | Contract identifier from IBKR's database. | [required] |
**ccy** | **String** | Contract Currency | [required] |
**exchange** | **String** | Exchange | [required] |
**qty** | **f64** | Order Quantity | [required] |
**r#type** | Option<**String**> | Order Price; required if order type is limit |  |
**side** | Option<**String**> | Side |  |
**price** | Option<**f64**> | Order Price; required if order type is limit |  |
**tif** | Option<**String**> | Time in Force |  |

### Return type

[**models::OrderData**](order-data.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ccp_order_put

> models::OrderData ccp_order_put(acct, id)
Update Order

Updates an Order. Updating an order requires the same arguments as placing an order besides the conid. Note: The status of the order can be queried through GET /ccp/order. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acct** | **String** | User Account | [required] |
**id** | **f64** | Order ID to be modified | [required] |

### Return type

[**models::OrderData**](order-data.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ccp_orders_get

> models::CcpOrdersGet200Response ccp_orders_get(acct, cancelled)
Order Status

Get status for all orders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**acct** | **String** | User Account | [required] |
**cancelled** | Option<**bool**> | Return only Rejected or Cancelled orders since today midnight |  |

### Return type

[**models::CcpOrdersGet200Response**](_ccp_orders_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ccp_positions_get

> models::PositionData ccp_positions_get()
Positions

List of positions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PositionData**](position-data.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ccp_status_get

> models::CcpStatusGet200Response ccp_status_get()
CCP Status

Provide the current CCP session status. When using the Gateway this endpoint will also initiate a brokerage session to CCP by sending /auth/init and response.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CcpStatusGet200Response**](_ccp_status_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ccp_trades_get

> models::CcpOrdersGet200Response ccp_trades_get(from, to)
Trades

Get a list of Trades, by default, the list is from today midnight to Date.now(). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> | From Date (YYYYMMDD-HH:mm:ss) or offset (-1,-2,-3..) |  |
**to** | Option<**String**> | To Date (YYYYMMDD-HH:mm:ss) or offset (-1,-2,-3..). To value should be bigger than from value.  |  |

### Return type

[**models::CcpOrdersGet200Response**](_ccp_orders_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

