# \ScannerApi

All URIs are relative to *http://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hmds_scanner_post**](ScannerApi.md#hmds_scanner_post) | **POST** /hmds/scanner | Run Scanner (Beta)
[**iserver_scanner_params_get**](ScannerApi.md#iserver_scanner_params_get) | **GET** /iserver/scanner/params | Scanner Parameters
[**iserver_scanner_run_post**](ScannerApi.md#iserver_scanner_run_post) | **POST** /iserver/scanner/run | Scanner Run



## hmds_scanner_post

> models::ScannerResult hmds_scanner_post(body)
Run Scanner (Beta)

Using a direct connection to the market data farm, will provide results to the requested scanner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**HmdsScannerPostRequest**](HmdsScannerPostRequest.md) | request body | [required] |

### Return type

[**models::ScannerResult**](scanner-result.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_scanner_params_get

> models::IserverScannerParamsGet200Response iserver_scanner_params_get()
Scanner Parameters

Returns an object contains four lists contain all parameters for scanners

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IserverScannerParamsGet200Response**](_iserver_scanner_params_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_scanner_run_post

> Vec<models::IserverScannerRunPost200ResponseInner> iserver_scanner_run_post(body)
Scanner Run

Searches for contracts according to the filters specified in scanner/params endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ScannerParams**](ScannerParams.md) | scanner-params request | [required] |

### Return type

[**Vec<models::IserverScannerRunPost200ResponseInner>**](_iserver_scanner_run_post_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

