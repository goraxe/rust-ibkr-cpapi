# \OrderApi

All URIs are relative to *http://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_account_id_order_order_id_delete**](OrderApi.md#iserver_account_account_id_order_order_id_delete) | **DELETE** /iserver/account/{accountId}/order/{orderId} | Cancel Order
[**iserver_account_account_id_order_order_id_post**](OrderApi.md#iserver_account_account_id_order_order_id_post) | **POST** /iserver/account/{accountId}/order/{orderId} | Modify Order
[**iserver_account_account_id_order_post**](OrderApi.md#iserver_account_account_id_order_post) | **POST** /iserver/account/{accountId}/order | Place Order (Deprecated)
[**iserver_account_account_id_order_whatif_post**](OrderApi.md#iserver_account_account_id_order_whatif_post) | **POST** /iserver/account/{accountId}/order/whatif | Preview Order (Deprecated)
[**iserver_account_account_id_orders_post**](OrderApi.md#iserver_account_account_id_orders_post) | **POST** /iserver/account/{accountId}/orders | Place Orders
[**iserver_account_account_id_orders_whatif_post**](OrderApi.md#iserver_account_account_id_orders_whatif_post) | **POST** /iserver/account/{accountId}/orders/whatif | Preview Orders
[**iserver_account_order_status_order_id_get**](OrderApi.md#iserver_account_order_status_order_id_get) | **GET** /iserver/account/order/status/{orderId} | Order Status
[**iserver_account_orders_fa_group_post**](OrderApi.md#iserver_account_orders_fa_group_post) | **POST** /iserver/account/orders/{faGroup} | Place Orders for FA
[**iserver_account_orders_get**](OrderApi.md#iserver_account_orders_get) | **GET** /iserver/account/orders | Live Orders
[**iserver_reply_replyid_post**](OrderApi.md#iserver_reply_replyid_post) | **POST** /iserver/reply/{replyid} | Place Order Reply



## iserver_account_account_id_order_order_id_delete

> models::IserverAccountAccountIdOrderOrderIdDelete200Response iserver_account_account_id_order_order_id_delete(account_id, order_id)
Cancel Order

Cancels an open order. Must call /iserver/accounts endpoint prior to cancelling an order. Use /iservers/account/orders endpoint to review open-order(s) and get latest order status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id, or fa group if deleting a group order | [required] |
**order_id** | **String** | Customer order id, use /iservers/account/orders endpoint to query orderId. | [required] |

### Return type

[**models::IserverAccountAccountIdOrderOrderIdDelete200Response**](_iserver_account__accountId__order__orderId__delete_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_order_order_id_post

> Vec<models::IserverAccountAccountIdOrderOrderIdPost200ResponseInner> iserver_account_account_id_order_order_id_post(account_id, order_id, body)
Modify Order

Modifies an open order. Must call /iserver/accounts endpoint prior to modifying an order. Use /iservers/account/orders endpoint to review open-order(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id, or fa group if modifying a group order | [required] |
**order_id** | **String** | Customer order id, use /iservers/account/orders endpoint to query orderId. | [required] |
**body** | [**ModifyOrder**](ModifyOrder.md) | modify-order request | [required] |

### Return type

[**Vec<models::IserverAccountAccountIdOrderOrderIdPost200ResponseInner>**](_iserver_account__accountId__order__orderId__post_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_order_post

> Vec<models::IserverAccountAccountIdOrderPost200ResponseInner> iserver_account_account_id_order_post(account_id, body)
Place Order (Deprecated)

This endpoint is going to be deprecated, you can use /iserver/account/{accountId}/orders, just pass one order in the array, the order structure will be same. Please note here, sometimes this endpoint alone can't make sure you submit the order successfully, you could receive some questions in the response, you have to to answer them in order to submit the order successfully. You can use \"/iserver/reply/{replyid}\" endpoint to answer questions 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**body** | [**OrderRequest**](OrderRequest.md) | order request info | [required] |

### Return type

[**Vec<models::IserverAccountAccountIdOrderPost200ResponseInner>**](_iserver_account__accountId__order_post_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_order_whatif_post

> models::IserverAccountAccountIdOrderWhatifPost200Response iserver_account_account_id_order_whatif_post(account_id, body)
Preview Order (Deprecated)

This end-point is going to be deprecated, you can use /iserver/account/{accountId}/orders/whatif, just pass one order in the array, the order structure will be same. This endpoint allows you to preview order without actually submitting the order and you can get commission information in the response. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**body** | [**OrderRequest**](OrderRequest.md) | order info | [required] |

### Return type

[**models::IserverAccountAccountIdOrderWhatifPost200Response**](_iserver_account__accountId__order_whatif_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_orders_post

> Vec<models::IserverAccountAccountIdOrderPost200ResponseInner> iserver_account_account_id_orders_post(account_id, body)
Place Orders

When connected to an IServer Brokerage Session, this endpoint will allow you to submit orders.  CP WEB API supports various advanced orderTypes, for additional details and examples refer to [IBKR Quant Blog](https://www.tradersinsight.news/category/ibkr-quant-news/programming_languages/rest-development/).   * Bracket - Attach additional opposite-side order(s) by using a single **cOID** sent with the parent and set the same value for **parentId** in each child order(s).   * Cash Quantity -  Send orders using monetary value by specifying **cashQty** instead of quantity, e.g. cashQty: 200. The endpoint /iserver/contract/rules returns list of valid orderTypes in cqtTypes.   * Currency Conversion - Convert cash from one currency to another by including **isCcyConv** = **true**. To specify the cash quantity use **fxQTY** instead of quantity, e.g. fxQTY: 100.   * Fractional - Contracts that support fractional shares can be traded by specifying **quantity** as a float, e.g. quantity: 0.001. The endpoint /iserver/contract/rules returns a list of valid orderTypes in fraqTypes.   * IB Algos - Attached user-defined settings to your trades by using any of IBKR's Algo Orders. Use the endpoint /iserver/contract/{conid}/algos to identify the available strategies for a contract.   * One-Cancels-All (OCA) - Group multiple unrelated orders by passing order request info in an array and including **isSingleGroup = true** for each order. All orders will be assigned the same oca_group_id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**body** | [**IserverAccountAccountIdOrdersPostRequest**](IserverAccountAccountIdOrdersPostRequest.md) | order request info | [required] |

### Return type

[**Vec<models::IserverAccountAccountIdOrderPost200ResponseInner>**](_iserver_account__accountId__order_post_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_orders_whatif_post

> models::IserverAccountAccountIdOrderWhatifPost200Response iserver_account_account_id_orders_whatif_post(account_id, body)
Preview Orders

This endpoint allows you to preview order without actually submitting the order and you can get commission information in the response. Also supports bracket orders. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**body** | [**IserverAccountAccountIdOrdersPostRequest**](IserverAccountAccountIdOrdersPostRequest.md) | order info | [required] |

### Return type

[**models::IserverAccountAccountIdOrderWhatifPost200Response**](_iserver_account__accountId__order_whatif_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_order_status_order_id_get

> models::OrderStatus iserver_account_order_status_order_id_get(order_id)
Order Status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | Customer order id, use /iservers/account/orders endpoint to query orderId. | [required] |

### Return type

[**models::OrderStatus**](order-status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_orders_fa_group_post

> Vec<models::IserverAccountAccountIdOrderPost200ResponseInner> iserver_account_orders_fa_group_post(fa_group, body)
Place Orders for FA

Financial Advisors can use this endpoint to place an order for a specified group. To place an order for a specified account use the endpoint /iserver/account/{accountId}/order. More information about groups can be found in the [TWS Users' Guide:](https://guides.interactivebrokers.com/twsguide/twsguide.htm#usersguidebook/financialadvisors/create_an_account_group_for_share_allocation.htm). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fa_group** | **String** | financial advisor group | [required] |
**body** | [**OrderRequest**](OrderRequest.md) | order request info | [required] |

### Return type

[**Vec<models::IserverAccountAccountIdOrderPost200ResponseInner>**](_iserver_account__accountId__order_post_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_orders_get

> models::IserverAccountOrdersGet200Response iserver_account_orders_get(filters)
Live Orders

The endpoint is meant to be used in polling mode, e.g. requesting every x seconds. The response will contain two objects, one is notification, the other is orders. Orders is the list of live orders (cancelled, filled, submitted). Notifications contains information about execute orders as they happen, see status field. To receive streaming live orders the endpoint /ws can be used. Refer to [Streaming WebSocket Data](https://interactivebrokers.github.io/cpwebapi/RealtimeSubscription.html) for details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | list of filters separated by comma |  |

### Return type

[**models::IserverAccountOrdersGet200Response**](_iserver_account_orders_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_reply_replyid_post

> Vec<models::IserverReplyReplyidPost200ResponseInner> iserver_reply_replyid_post(replyid, body)
Place Order Reply

Reply to questions when placing orders and submit orders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**replyid** | **String** | Please use the \"id\" from the response of \"Place Order\" endpoint | [required] |
**body** | [**IserverReplyReplyidPostRequest**](IserverReplyReplyidPostRequest.md) | Answer to question | [required] |

### Return type

[**Vec<models::IserverReplyReplyidPost200ResponseInner>**](_iserver_reply__replyid__post_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

