# \ContractApi

All URIs are relative to *http://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_contract_conid_algos_get**](ContractApi.md#iserver_contract_conid_algos_get) | **GET** /iserver/contract/{conid}/algos | IB Algo Params
[**iserver_contract_conid_info_and_rules_get**](ContractApi.md#iserver_contract_conid_info_and_rules_get) | **GET** /iserver/contract/{conid}/info-and-rules | Info and Rules
[**iserver_contract_conid_info_get**](ContractApi.md#iserver_contract_conid_info_get) | **GET** /iserver/contract/{conid}/info | Contract Details
[**iserver_contract_rules_post**](ContractApi.md#iserver_contract_rules_post) | **POST** /iserver/contract/rules | Contract Rules
[**iserver_secdef_info_get**](ContractApi.md#iserver_secdef_info_get) | **GET** /iserver/secdef/info | Secdef Info
[**iserver_secdef_search_post**](ContractApi.md#iserver_secdef_search_post) | **POST** /iserver/secdef/search | Search by Symbol or Name
[**iserver_secdef_strikes_get**](ContractApi.md#iserver_secdef_strikes_get) | **GET** /iserver/secdef/strikes | Search Strikes
[**trsrv_futures_get**](ContractApi.md#trsrv_futures_get) | **GET** /trsrv/futures | Security Futures by Symbol
[**trsrv_secdef_post**](ContractApi.md#trsrv_secdef_post) | **POST** /trsrv/secdef | Secdef by Conid
[**trsrv_secdef_schedule_get**](ContractApi.md#trsrv_secdef_schedule_get) | **GET** /trsrv/secdef/schedule | Get trading schedule for symbol
[**trsrv_stocks_get**](ContractApi.md#trsrv_stocks_get) | **GET** /trsrv/stocks | Security Stocks by Symbol



## iserver_contract_conid_algos_get

> Vec<models::IserverContractConidAlgosGet200ResponseInner> iserver_contract_conid_algos_get(conid, algos, add_description, add_params)
IB Algo Params

Returns supported IB Algos for contract. Must be called a second time to query the list of available parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** | IBKR contract identifier | [required] |
**algos** | Option<**String**> | List of algo ids delimited by \";\" to filter by. Max of 8 algos ids can be specified. |  |
**add_description** | Option<**String**> | Whether or not to add algo descriptions to response. Set to 1 for yes, 0 for no. |  |
**add_params** | Option<**String**> | Whether or not to show algo parameters.  Set to 1 for yes, 0 for no. |  |

### Return type

[**Vec<models::IserverContractConidAlgosGet200ResponseInner>**](_iserver_contract__conid__algos_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_contract_conid_info_and_rules_get

> models::IserverContractConidInfoAndRulesGet200Response iserver_contract_conid_info_and_rules_get(conid, is_buy)
Info and Rules

Returns both contract info and rules from a single endpoint. For only contract rules, use the endpoint /iserver/contract/rules. For only contract info, use the endpoint /iserver/contract/{conid}/info.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** | IBKR contract identifier | [required] |
**is_buy** | **bool** | Side of the market rules apply too. Set to **true** for Buy Orders, set to **false** for Sell Orders | [required] |

### Return type

[**models::IserverContractConidInfoAndRulesGet200Response**](_iserver_contract__conid__info_and_rules_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_contract_conid_info_get

> models::Contract iserver_contract_conid_info_get(conid)
Contract Details

Using the Contract Identifier get contract info. You can use this to prefill your order before you submit an order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** | contract id | [required] |

### Return type

[**models::Contract**](contract.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_contract_rules_post

> models::IserverContractRulesPost200Response iserver_contract_rules_post(conid)
Contract Rules

Returns trading related rules for a specific contract and side. For both contract info and rules use the endpoint /iserver/contract/{conid}/info-and-rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | [**IserverContractRulesPostRequest**](IserverContractRulesPostRequest.md) |  | [required] |

### Return type

[**models::IserverContractRulesPost200Response**](_iserver_contract_rules_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_secdef_info_get

> Vec<models::SecdefInfo> iserver_secdef_info_get(conid, sectype, month, exchange, strike, right)
Secdef Info

Provides Contract Details of Futures, Options, Warrants, Cash and CFDs based on conid. To get the strike price for Options/Warrants use \"/iserver/secdef/strikes\" endpoint. Must call /secdef/search for the underlying contract first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** | underlying contract id | [required] |
**sectype** | **String** | FUT/OPT/WAR/CASH/CFD | [required] |
**month** | Option<**String**> | contract month, only required for FUT/OPT/WAR in the format MMMYY, example JAN00 |  |
**exchange** | Option<**String**> | optional, default is SMART |  |
**strike** | Option<**f64**> | optional, only required for OPT/WAR |  |
**right** | Option<**String**> | C for call, P for put |  |

### Return type

[**Vec<models::SecdefInfo>**](secdef-info.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_secdef_search_post

> Vec<models::IserverSecdefSearchPost200ResponseInner> iserver_secdef_search_post(symbol)
Search by Symbol or Name

Search by underlying symbol or company name. Relays back what derivative contract(s) it has. This endpoint must be called before using /secdef/info. If company name is specified will only receive limited response: conid, companyName, companyHeader and symbol. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | [**IserverSecdefSearchPostRequest**](IserverSecdefSearchPostRequest.md) | Symbol or Company Name to be searched | [required] |

### Return type

[**Vec<models::IserverSecdefSearchPost200ResponseInner>**](_iserver_secdef_search_post_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_secdef_strikes_get

> models::IserverSecdefStrikesGet200Response iserver_secdef_strikes_get(conid, sectype, month, exchange)
Search Strikes

Query strikes for Options/Warrants. For the conid of the underlying contract, available contract months and exchanges use \"/iserver/secdef/search\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** | contract id of the underlying contract | [required] |
**sectype** | **String** | OPT/WAR | [required] |
**month** | **String** | contract month | [required] |
**exchange** | Option<**String**> | optional, default is SMART |  |

### Return type

[**models::IserverSecdefStrikesGet200Response**](_iserver_secdef_strikes_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trsrv_futures_get

> models::TrsrvFuturesGet200Response trsrv_futures_get(symbols)
Security Futures by Symbol

Returns a list of non-expired future contracts for given symbol(s)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbols** | **String** | list of case-sensitive symbols separated by comma | [required] |

### Return type

[**models::TrsrvFuturesGet200Response**](_trsrv_futures_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trsrv_secdef_post

> Vec<models::SecdefInner> trsrv_secdef_post(body)
Secdef by Conid

Returns a list of security definitions for the given conids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TrsrvSecdefPostRequest**](TrsrvSecdefPostRequest.md) | request body | [required] |

### Return type

[**Vec<models::SecdefInner>**](secdef_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trsrv_secdef_schedule_get

> models::TrsrvSecdefScheduleGet200Response trsrv_secdef_schedule_get(asset_class, symbol, exchange, exchange_filter)
Get trading schedule for symbol

Returns the trading schedule up to a month for the requested contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_class** | **String** | specify the asset class of the contract. Available values-- Stock: STK, Option: OPT, Future: FUT, Contract For Difference: CFD, Warrant: WAR, Forex: SWP, Mutual Fund: FND, Bond: BND, Inter-Commodity Spreads: ICS  | [required] |
**symbol** | **String** | Underlying Symbol for specified contract, for example 'AAPL' for US Stock - Apple Inc. | [required] |
**exchange** | Option<**String**> | Native exchange for contract, for example 'NASDAQ' for US Stock - Apple Inc. |  |
**exchange_filter** | Option<**String**> | Response only returns trading schedule for specified exchange |  |

### Return type

[**models::TrsrvSecdefScheduleGet200Response**](_trsrv_secdef_schedule_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trsrv_stocks_get

> models::TrsrvStocksGet200Response trsrv_stocks_get(symbols)
Security Stocks by Symbol

Returns an object contains all stock contracts for given symbol(s)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbols** | **String** | list of upper-sensitive symbols separated by comma | [required] |

### Return type

[**models::TrsrvStocksGet200Response**](_trsrv_stocks_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

