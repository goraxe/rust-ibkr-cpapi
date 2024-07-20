# HistoryResultBars

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**open** | Option<**f64**> | First price returned for bar value. | [optional]
**start_time** | Option<**String**> | Start Time in the format YYYYMMDD. | [optional]
**start_time_val** | Option<**i32**> | Start Time Value - Formatted in unix time in ms. | [optional]
**end_time** | Option<**String**> | End Time in the format YYYYMMDD. | [optional]
**end_time_val** | Option<**i32**> | End Time Value - Formatted in unix time in ms. | [optional]
**points** | Option<**i32**> | total number of data points. | [optional]
**data** | Option<[**Vec<models::HistoryResultBarsDataInner>**](history_result_bars_data_inner.md)> |  | [optional]
**mkt_data_delay** | Option<**i32**> | If 0 then data is returned in real time. Otherwise will return the number of seconds history data is delayed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


