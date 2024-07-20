# IserverContractRulesPost200ResponseRulesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**algo_eligible** | Option<**bool**> |  | [optional]
**can_trade_acct_ids** | Option<[**Vec<models::IserverContractRulesPost200ResponseRulesInnerCanTradeAcctIdsInner>**](_iserver_contract_rules_post_200_response_rules_inner_canTradeAcctIds_inner.md)> |  | [optional]
**error** | Option<**String**> | Returns a description on any errors with order presets | [optional]
**order_types** | Option<[**Vec<models::IserverContractRulesPost200ResponseRulesInnerOrderTypesInner>**](_iserver_contract_rules_post_200_response_rules_inner_orderTypes_inner.md)> |  | [optional]
**ibalgo_types** | Option<[**Vec<models::IserverContractRulesPost200ResponseRulesInnerIbalgoTypesInner>**](_iserver_contract_rules_post_200_response_rules_inner_ibalgoTypes_inner.md)> |  | [optional]
**fraq_types** | Option<[**Vec<models::IserverContractRulesPost200ResponseRulesInnerFraqTypesInner>**](_iserver_contract_rules_post_200_response_rules_inner_fraqTypes_inner.md)> |  | [optional]
**cqt_types** | Option<[**Vec<models::IserverContractRulesPost200ResponseRulesInnerCqtTypesInner>**](_iserver_contract_rules_post_200_response_rules_inner_cqtTypes_inner.md)> |  | [optional]
**order_defaults** | Option<[**Vec<models::IserverContractRulesPost200ResponseRulesInnerOrderDefaultsInner>**](_iserver_contract_rules_post_200_response_rules_inner_orderDefaults_inner.md)> | If object returned will provide the defaults based on user settings | [optional]
**order_types_outside** | Option<[**Vec<models::IserverContractRulesPost200ResponseRulesInnerOrderTypesOutsideInner>**](_iserver_contract_rules_post_200_response_rules_inner_orderTypesOutside_inner.md)> |  | [optional]
**default_size** | Option<**i32**> | Default quantity | [optional]
**cash_size** | Option<**i32**> | cash value | [optional]
**size_increment** | Option<**i32**> | increment quantity value | [optional]
**tif_types** | Option<[**Vec<models::IserverContractRulesPost200ResponseRulesInnerTifTypesInner>**](_iserver_contract_rules_post_200_response_rules_inner_tifTypes_inner.md)> |  | [optional]
**default_tif** | Option<**String**> | Default time in force value | [optional]
**limit_price** | Option<**f64**> | Limit price | [optional]
**stopprice** | Option<**f64**> | Stop price | [optional]
**order_origination** | Option<**f64**> | Order origin designation for US securities options and Options Clearing Corporation | [optional]
**preview** | Option<**bool**> | order preview required | [optional]
**display_size** | Option<**f64**> |  | [optional]
**fraq_int** | Option<**f64**> | decimal places for fractional order size | [optional]
**cash_ccy** | Option<**String**> | Cash currency for the contract | [optional]
**cash_qty_incr** | Option<**f64**> | Increment value for cash quantity | [optional]
**price_magnifier** | Option<**f64**> | Price Magnifier | [optional]
**negative_capable** | Option<**bool**> | trading negative price support | [optional]
**increment** | Option<**f64**> | Price increment value | [optional]
**increment_digits** | Option<**i32**> | Number of digits for price increment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


