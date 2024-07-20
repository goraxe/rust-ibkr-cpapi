# SsoValidateGet200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**login_type** | Option<**f64**> | 1 for Live, 2 for Paper | [optional]
**user_name** | Option<**String**> | Username | [optional]
**user_id** | Option<**f64**> | User ID | [optional]
**expire** | Option<**f64**> | Time in milliseconds until session expires. Caller needs to call the again to re-validate session | [optional]
**result** | Option<**bool**> | true if session was validated; false if not. | [optional]
**auth_time** | Option<**f64**> | Time of session validation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


