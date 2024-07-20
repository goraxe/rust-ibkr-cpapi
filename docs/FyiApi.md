# \FyiApi

All URIs are relative to *http://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fyi_deliveryoptions_device_id_delete**](FyiApi.md#fyi_deliveryoptions_device_id_delete) | **DELETE** /fyi/deliveryoptions/{deviceId} | Delete a device
[**fyi_deliveryoptions_device_post**](FyiApi.md#fyi_deliveryoptions_device_post) | **POST** /fyi/deliveryoptions/device | Enable/Disable device option
[**fyi_deliveryoptions_email_put**](FyiApi.md#fyi_deliveryoptions_email_put) | **PUT** /fyi/deliveryoptions/email | Enable/Disable email option
[**fyi_deliveryoptions_get**](FyiApi.md#fyi_deliveryoptions_get) | **GET** /fyi/deliveryoptions | Get delivery options
[**fyi_disclaimer_typecode_get**](FyiApi.md#fyi_disclaimer_typecode_get) | **GET** /fyi/disclaimer/{typecode} | Get disclaimer for a certain kind of fyi
[**fyi_disclaimer_typecode_put**](FyiApi.md#fyi_disclaimer_typecode_put) | **PUT** /fyi/disclaimer/{typecode} | Mark disclaimer read
[**fyi_notifications_get**](FyiApi.md#fyi_notifications_get) | **GET** /fyi/notifications | Get a list of notifications
[**fyi_notifications_more_get**](FyiApi.md#fyi_notifications_more_get) | **GET** /fyi/notifications/more | Get more notifications based on a certain one
[**fyi_notifications_notification_id_put**](FyiApi.md#fyi_notifications_notification_id_put) | **PUT** /fyi/notifications/{notificationId} | Get a list of notifications
[**fyi_settings_get**](FyiApi.md#fyi_settings_get) | **GET** /fyi/settings | Get a list of subscriptions
[**fyi_settings_typecode_post**](FyiApi.md#fyi_settings_typecode_post) | **POST** /fyi/settings/{typecode} | Enable/Disable certain subscription
[**fyi_unreadnumber_get**](FyiApi.md#fyi_unreadnumber_get) | **GET** /fyi/unreadnumber | Get unread number of fyis. The HTTP method POST is also supported.



## fyi_deliveryoptions_device_id_delete

> serde_json::Value fyi_deliveryoptions_device_id_delete(device_id)
Delete a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | device ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_deliveryoptions_device_post

> models::FyiDisclaimerTypecodePut200Response fyi_deliveryoptions_device_post(body)
Enable/Disable device option

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FyiDeliveryoptionsDevicePostRequest**](FyiDeliveryoptionsDevicePostRequest.md) | device info | [required] |

### Return type

[**models::FyiDisclaimerTypecodePut200Response**](_fyi_disclaimer__typecode__put_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_deliveryoptions_email_put

> models::FyiDisclaimerTypecodePut200Response fyi_deliveryoptions_email_put(enabled)
Enable/Disable email option

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enabled** | **String** | true/false | [required] |

### Return type

[**models::FyiDisclaimerTypecodePut200Response**](_fyi_disclaimer__typecode__put_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_deliveryoptions_get

> models::FyiDeliveryoptionsGet200Response fyi_deliveryoptions_get()
Get delivery options

options for sending fyis to email and other devices 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::FyiDeliveryoptionsGet200Response**](_fyi_deliveryoptions_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_disclaimer_typecode_get

> models::FyiDisclaimerTypecodeGet200Response fyi_disclaimer_typecode_get(typecode)
Get disclaimer for a certain kind of fyi

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**typecode** | **String** | fyi code, for example --M8, EA | [required] |

### Return type

[**models::FyiDisclaimerTypecodeGet200Response**](_fyi_disclaimer__typecode__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_disclaimer_typecode_put

> models::FyiDisclaimerTypecodePut200Response fyi_disclaimer_typecode_put(typecode)
Mark disclaimer read

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**typecode** | **String** | fyi code, for example --M8, EA | [required] |

### Return type

[**models::FyiDisclaimerTypecodePut200Response**](_fyi_disclaimer__typecode__put_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_notifications_get

> Vec<models::NotificationsInner> fyi_notifications_get(max, exclude, include)
Get a list of notifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | **String** | max number of fyis in response | [required] |
**exclude** | Option<**String**> | if set, don't set include |  |
**include** | Option<**String**> | if set, don't set exclude |  |

### Return type

[**Vec<models::NotificationsInner>**](notifications_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_notifications_more_get

> Vec<models::NotificationsInner> fyi_notifications_more_get(id)
Get more notifications based on a certain one

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | id of last notification in the list | [required] |

### Return type

[**Vec<models::NotificationsInner>**](notifications_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_notifications_notification_id_put

> serde_json::Value fyi_notifications_notification_id_put(notification_id)
Get a list of notifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | mark a notification read | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_settings_get

> Vec<models::FyiSettingsGet200ResponseInner> fyi_settings_get()
Get a list of subscriptions

Return the current choices of subscriptions, we can toggle the option 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FyiSettingsGet200ResponseInner>**](_fyi_settings_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_settings_typecode_post

> serde_json::Value fyi_settings_typecode_post(typecode, body)
Enable/Disable certain subscription

Configure which typecode you would like to enable/disable. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**typecode** | **String** | fyi code | [required] |
**body** | [**FyiSettingsTypecodePostRequest**](FyiSettingsTypecodePostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_unreadnumber_get

> models::FyiUnreadnumberGet200Response fyi_unreadnumber_get()
Get unread number of fyis. The HTTP method POST is also supported.

Returns the total number of unread fyis 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::FyiUnreadnumberGet200Response**](_fyi_unreadnumber_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

