# \MarketDataApi

All URIs are relative to *http://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**hmds_history_get**](MarketDataApi.md#hmds_history_get) | **GET** /hmds/history | Market Data History (Beta)
[**iserver_marketdata_conid_unsubscribe_get**](MarketDataApi.md#iserver_marketdata_conid_unsubscribe_get) | **GET** /iserver/marketdata/{conid}/unsubscribe | Market Data Cancel (Single)
[**iserver_marketdata_history_get**](MarketDataApi.md#iserver_marketdata_history_get) | **GET** /iserver/marketdata/history | Market Data History
[**iserver_marketdata_snapshot_get**](MarketDataApi.md#iserver_marketdata_snapshot_get) | **GET** /iserver/marketdata/snapshot | Market Data
[**iserver_marketdata_unsubscribeall_get**](MarketDataApi.md#iserver_marketdata_unsubscribeall_get) | **GET** /iserver/marketdata/unsubscribeall | Market Data Cancel (All)
[**md_snapshot_get**](MarketDataApi.md#md_snapshot_get) | **GET** /md/snapshot | Market Data Snapshot (Beta)



## hmds_history_get

> models::HistoryResult hmds_history_get(conid, period, bar, outside_rth)
Market Data History (Beta)

Using a direct connection to the market data farm, will provide a list of historical market data for given conid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **i32** | contract id | [required] |
**period** | **String** | Time period for history request.    * min: Minutes   * h: Hours   * d: Days   * w: Weeks   * m: Months   * y: Years  | [required] |
**bar** | Option<**String**> | Duration of time for each candlestick bar.   * min: Minutes   * h: Hours   * d: Days   * w: Weeks   * m: Months  |  |
**outside_rth** | Option<**bool**> | For contracts that support it, will determine if history data includes outside of regular trading hours. |  |

### Return type

[**models::HistoryResult**](history-result.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_marketdata_conid_unsubscribe_get

> models::IserverMarketdataConidUnsubscribeGet200Response iserver_marketdata_conid_unsubscribe_get(conid)
Market Data Cancel (Single)

Cancel market data for given conid. To cancel all market data request(s), see /iserver/marketdata/unsubscribeall. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** | contract id | [required] |

### Return type

[**models::IserverMarketdataConidUnsubscribeGet200Response**](_iserver_marketdata__conid__unsubscribe_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_marketdata_history_get

> models::HistoryData iserver_marketdata_history_get(conid, period, exchange, bar, outside_rth)
Market Data History

Get historical market Data for given conid, length of data is controlled by 'period' and 'bar'. Formatted as: min=minute, h=hour, d=day, w=week, m=month, y=year e.g. period =1y with bar =1w returns 52 data points (Max of 1000 data points supported). **Note**: There's a limit of 5 concurrent requests. Excessive requests will return a 'Too many requests' status 429 response. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** | contract id | [required] |
**period** | **String** | available time period-- {1-30}min, {1-8}h, {1-1000}d, {1-792}w, {1-182}m, {1-15}y | [required] |
**exchange** | Option<**String**> | Exchange of the conid (e.g. ISLAND, NYSE, etc.). Default value is empty which corresponds to primary exchange of the conid. |  |
**bar** | Option<**String**> | possible value-- 1min, 2min, 3min, 5min, 10min, 15min, 30min, 1h, 2h, 3h, 4h, 8h, 1d, 1w, 1m |  |
**outside_rth** | Option<**bool**> | For contracts that support it, will determine if historical data includes outside of regular trading hours. |  |

### Return type

[**models::HistoryData**](history-data.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_marketdata_snapshot_get

> Vec<models::IserverMarketdataSnapshotGet200ResponseInner> iserver_marketdata_snapshot_get(conids, since, fields)
Market Data

Get Market Data for the given conid(s). The endpoint will return by default bid, ask, last, change, change pct, close, listing exchange. See response fields for a list of available fields that can be request via fields argument. The endpoint /iserver/accounts must be called prior to /iserver/marketdata/snapshot. For derivative contracts the endpoint /iserver/secdef/search must be called first. First /snapshot endpoint call for given conid will initiate the market data request.  To receive all available fields the /snapshot endpoint will need to be called several times. To receive streaming market data the endpoint /ws can be used. Refer to [Streaming WebSocket Data](https://interactivebrokers.github.io/cpwebapi/RealtimeSubscription.html) for details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conids** | **String** | list of conids separated by comma | [required] |
**since** | Option<**i32**> | time period since which updates are required. uses epoch time with milliseconds. |  |
**fields** | Option<**String**> | list of fields separated by comma |  |

### Return type

[**Vec<models::IserverMarketdataSnapshotGet200ResponseInner>**](_iserver_marketdata_snapshot_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_marketdata_unsubscribeall_get

> models::IserverMarketdataUnsubscribeallGet200Response iserver_marketdata_unsubscribeall_get()
Market Data Cancel (All)

Cancel all market data request(s). To cancel market data for given conid, see /iserver/marketdata/{conid}/unsubscribe. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IserverMarketdataUnsubscribeallGet200Response**](_iserver_marketdata_unsubscribeall_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## md_snapshot_get

> models::MarketData md_snapshot_get(conids, fields)
Market Data Snapshot (Beta)

Get a snapshot of Market Data for the given conid(s).See response for a list of available fields that can be requested from the fields argument. Must be connected to a brokerage session before can query snapshot data. First /snapshot endpoint call for given conid(s) will initiate the market data request, make an additional request to receive field values back. To receive all available fields the /snapshot endpoint will need to be called several times. To receive streaming market data the endpoint /ws can be used. Refer to [Streaming WebSocket Data](https://interactivebrokers.github.io/cpwebapi/RealtimeSubscription.html) for details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conids** | **String** | List of conids comma separated. Optional exchange and instrument type can be specified.   * conid: IBKR Contract Identifier   * exchange: Exchange or venue   * instrType: Instrument Type supported values: CS (Stocks), OPT (Options), FUT (Futures), FOP (Future Options), WAR (Warrants), BOND (Bonds), FUND (Mutual Funds), CASH (Forex), CFD (Contract for difference), IND (Index)  | [required] |
**fields** | Option<**String**> | list of fields separated by comma |  |

### Return type

[**models::MarketData**](market-data.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

