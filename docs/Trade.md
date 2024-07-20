# Trade

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**execution_id** | Option<**String**> | execution identifier for the order | [optional]
**symbol** | Option<**String**> | Underlying Symbol | [optional]
**side** | Option<**String**> | The side of the market of the order.   * B - Buy contract near posted ask price   * S - Sell contract near posted bid price   * X - Option expired  | [optional]
**order_description** | Option<**String**> | Formatted description of the order \"%side% %size% @ %price% on %exchange%\". | [optional]
**trade_time** | Option<**String**> | Time of Status update in format \"YYYYMMDD-hh:mm:ss\". | [optional]
**trade_time_r** | Option<**f64**> | Time of status update in format unix time. | [optional]
**size** | Option<**String**> | Quantity of the order | [optional]
**price** | Option<**String**> | Average Price | [optional]
**order_ref** | Option<**String**> | User defined string used to identify the order. Value is set using \"cOID\" field while placing an order. | [optional]
**submitter** | Option<**String**> | User that submitted order | [optional]
**exchange** | Option<**String**> | Exchange or venue of order | [optional]
**commission** | Option<**f64**> | Commission of the order | [optional]
**net_amount** | Option<**f64**> | Net cost of the order, including contract multiplier and quantity. | [optional]
**account** | Option<**String**> | accountCode | [optional]
**acount_code** | Option<**String**> | Account Number | [optional]
**company_name** | Option<**String**> | Contracts company name | [optional]
**contract_description_1** | Option<**String**> | Format contract name | [optional]
**sec_type** | Option<**String**> | Asset class | [optional]
**conid** | Option<**String**> | IBKR's contract identifier | [optional]
**conidex** | Option<**String**> | conid and exchange. Format supports conid or conid@exchange | [optional]
**position** | Option<**String**> | Total quantity owned for this contract | [optional]
**clearing_id** | Option<**String**> | Firm which will settle the trade. For IBExecution customers only. | [optional]
**clearing_name** | Option<**String**> | Specifies the true beneficiary of the order. For IBExecution customers only. | [optional]
**liquidation_trade** | Option<**f64**> | If order adds liquidity to the market. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


