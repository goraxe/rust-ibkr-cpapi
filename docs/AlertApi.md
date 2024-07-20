# \AlertApi

All URIs are relative to *http://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_account_id_alert_activate_post**](AlertApi.md#iserver_account_account_id_alert_activate_post) | **POST** /iserver/account/{accountId}/alert/activate | Activate or deactivate an alert
[**iserver_account_account_id_alert_alert_id_delete**](AlertApi.md#iserver_account_account_id_alert_alert_id_delete) | **DELETE** /iserver/account/{accountId}/alert/{alertId} | Delete an alert
[**iserver_account_account_id_alert_post**](AlertApi.md#iserver_account_account_id_alert_post) | **POST** /iserver/account/{accountId}/alert | Create or modify alert
[**iserver_account_account_id_alerts_get**](AlertApi.md#iserver_account_account_id_alerts_get) | **GET** /iserver/account/{accountId}/alerts | Get a list of available alerts
[**iserver_account_alert_id_get**](AlertApi.md#iserver_account_alert_id_get) | **GET** /iserver/account/alert/{id} | Get details of an alert
[**iserver_account_mta_get**](AlertApi.md#iserver_account_mta_get) | **GET** /iserver/account/mta | Get MTA alert



## iserver_account_account_id_alert_activate_post

> models::IserverAccountAccountIdAlertActivatePost200Response iserver_account_account_id_alert_activate_post(account_id, body)
Activate or deactivate an alert

Please note, if alertId is 0, it will activate/deactivate all alerts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**body** | [**IserverAccountAccountIdAlertActivatePostRequest**](IserverAccountAccountIdAlertActivatePostRequest.md) | order request info | [required] |

### Return type

[**models::IserverAccountAccountIdAlertActivatePost200Response**](_iserver_account__accountId__alert_activate_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_alert_alert_id_delete

> models::IserverAccountAccountIdAlertActivatePost200Response iserver_account_account_id_alert_alert_id_delete(account_id, alert_id)
Delete an alert

Please be careful, if alertId is 0, it will delete all alerts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**alert_id** | **String** | alert id | [required] |

### Return type

[**models::IserverAccountAccountIdAlertActivatePost200Response**](_iserver_account__accountId__alert_activate_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_alert_post

> models::IserverAccountAccountIdAlertPost200Response iserver_account_account_id_alert_post(account_id, body)
Create or modify alert

Please note here, DO NOT pass orderId when creating a new alert, toolId is only required for MTA alert 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**body** | [**AlertRequest**](AlertRequest.md) | alert info | [required] |

### Return type

[**models::IserverAccountAccountIdAlertPost200Response**](_iserver_account__accountId__alert_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_alerts_get

> Vec<models::IserverAccountAccountIdAlertsGet200ResponseInner> iserver_account_account_id_alerts_get(account_id)
Get a list of available alerts

The response will contain both active and inactive alerts, but it won't have MTA alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |

### Return type

[**Vec<models::IserverAccountAccountIdAlertsGet200ResponseInner>**](_iserver_account__accountId__alerts_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_alert_id_get

> models::AlertResponse iserver_account_alert_id_get(id)
Get details of an alert

Use the endpoint /iserver/account//alerts to receive the alert id. The order_id in the response is the alert id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | alert id | [required] |

### Return type

[**models::AlertResponse**](alert-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_mta_get

> models::AlertResponse iserver_account_mta_get()
Get MTA alert

Each login user only has one mobile trading assistant (MTA) alert with it's own unique tool id. The tool id cannot be changed. When modified a new order Id is generated. MTA alerts can not be created or deleted. If you call delete /iserver/account//alert/:alertId, it will reset MTA to default. See [here](https://www.interactivebrokers.com/en/software/mobileiphonemobile/mobileiphone.htm#monitor/trading-assistant.htm) for more information on MTA alerts. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AlertResponse**](alert-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

