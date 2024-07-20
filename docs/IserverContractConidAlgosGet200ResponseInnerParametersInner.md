# IserverContractConidAlgosGet200ResponseInnerParametersInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The algo parameter | 
**required** | Option<**bool**> | If true a value must be entered. | [optional]
**name** | Option<**String**> | Descriptive name of the parameter. | [optional]
**value_class_name** | **String** | Format of the parameter. | 
**min_value** | Option<**f64**> | Smallest value, only applies to parameters with valueClassName=Double. | [optional]
**max_value** | Option<**f64**> | Largest value, only applies to parameters with valueClassName=Double. | [optional]
**default_value** | Option<**bool**> | User configured preset for this parameter. | [optional]
**legal_strings** | Option<**String**> | The list of choices | [optional]
**description** | Option<**String**> | Detailed description of the parameter. | [optional]
**gui_rank** | Option<**f64**> | The order in UI, used when building dynamic UI so that more important parameters are presented first. | [optional]
**price_market_rule** | Option<**bool**> | If true, must specify parameter using market rule format. Only applies to parameters with valueClassName=Double. | [optional]
**enabled_conditions** | Option<**String**> | The rules that UI should apply to algo parameters depending on chosen order type:  * MKT:speedUp:=:no - hide SpeedUp param when MKT is chosen for order type.  * LMT:strategyType:<>:empty - strategyType param cannot be empty when LMT is chosen for order type.  * MKT:strategyType:=:Marketable - set strategyType param to Marketable and disable (no other choice) when MKT is chosen for order type.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


