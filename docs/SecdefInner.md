# SecdefInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conid** | Option<**i32**> | IBKR contract identifier. | [optional]
**currency** | Option<**String**> | Currency contract trades in. | [optional]
**cross_currency** | Option<**bool**> | Defines if a derivative contract has a different currency. | [optional]
**time** | Option<**i32**> |  | [optional]
**chinese_name** | Option<**String**> | HTML encoded company description in Chinese. | [optional]
**all_exchanges** | Option<**String**> | List of exchanges and venues contract trades. | [optional]
**listing_exchange** | Option<**String**> | Main trading venue. | [optional]
**name** | Option<**String**> | Company Name. | [optional]
**asset_class** | Option<**String**> | Group of financial instruments which have similar financial characteristics and behave similar in the marketplace. | [optional]
**expiry** | Option<**String**> | Specific data contract expires. | [optional]
**last_trading_day** | Option<**String**> | Final day derivative contract can be traded before delivery of the underlying asset or cash settlement. | [optional]
**group** | Option<**String**> | Potential characteristic of each product. | [optional]
**put_or_call** | Option<**String**> | Defines the right to buy or sell of the underlying security. | [optional]
**sector** | Option<**String**> | The category of the economy. | [optional]
**sector_group** | Option<**String**> | Stock Group contract belongs too. | [optional]
**strike** | Option<**f64**> | Set price at which a derivative contract can be bought or sold. | [optional]
**ticker** | Option<**String**> | Contract symbol. | [optional]
**und_conid** | Option<**i32**> | Underlying contract identifier. | [optional]
**multiplier** | Option<**i32**> | Multiplier for total premium paid or received for derivative contract. | [optional]
**r#type** | Option<**String**> | Stock type. | [optional]
**und_comp** | Option<**String**> | Company name for underlying contract. | [optional]
**und_sym** | Option<**String**> | IBKR Symbol for underlying contract. | [optional]
**has_options** | Option<**bool**> | If contract has an option. | [optional]
**full_name** | Option<**String**> | Formatted company name with underlying symbol, expiration, strike, right. | [optional]
**is_us** | Option<**bool**> | If contract is a US contract. Currently supported for stocks, options and warrants. | [optional]
**increment_rules** | Option<[**models::SecdefInnerIncrementRules**](secdef_inner_incrementRules.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


