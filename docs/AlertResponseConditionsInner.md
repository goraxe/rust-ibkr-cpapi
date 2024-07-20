# AlertResponseConditionsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**condition_type** | Option<**i32**> | Types: 1-Price, 3-Time, 4-Margin, 5-Trade, 6-Volume, 7: MTA market 8: MTA Position, 9: MTA Acc. Daily PN&  | [optional]
**conidex** | Option<**String**> | conid and exchange. Format supports conid or conid@exchange | [optional]
**contract_description_1** | Option<**String**> | Format contract name | [optional]
**condition_operator** | Option<**String**> | optional, operator for the current condition   * >= Greater than or equal to   * <= Less than or equal to  | [optional]
**condition_trigger_method** | Option<**String**> | optional, only some type of conditions have triggerMethod | [optional]
**condition_value** | Option<**String**> | can not be empty, can pass default value \"*\" | [optional]
**condition_logic_bind** | Option<**String**> | Condition array should end with \"n\"   * a - AND   * o - OR   * n - END  | [optional]
**condition_time_zone** | Option<**String**> | only needed for some MTA alert condition | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


