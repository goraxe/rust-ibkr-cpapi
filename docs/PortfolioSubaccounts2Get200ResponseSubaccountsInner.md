# PortfolioSubaccounts2Get200ResponseSubaccountsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The account identification value | [optional]
**account_id** | Option<**String**> | The account number | [optional]
**account_van** | Option<**String**> | The accountAlias | [optional]
**account_title** | Option<**String**> | Title of the account | [optional]
**display_name** | Option<**String**> | Whichever value is not null in this priority | [optional]
**account_alias** | Option<**String**> | User customizable account alias. Refer to [Configure Account Alias](https://guides.interactivebrokers.com/cp/cp.htm#am/settings/accountalias.htm) for details. | [optional]
**account_status** | Option<**f64**> | When the account was opened in unix time. | [optional]
**currency** | Option<**String**> | Base currency of the account. | [optional]
**r#type** | Option<**String**> | Account Type | [optional]
**trading_type** | Option<**String**> | UNI - Deprecated property | [optional]
**faclient** | Option<**bool**> | If an account is a sub-account to a Financial Advisor. | [optional]
**clearing_status** | Option<**String**> | Status of the Account   * O = Open   * P or N = Pending   * A = Abandoned   * R = Rejected   * C = Closed   covestor:     type: boolean     description: Is a Covestor Account   parent:     type: object     properties:       mmc:         type: array         items:           type: string           description: Money Manager Client (MMC) Account       accountId:         type: string         description: Account Number for Money Manager Client       isMParent:         type: boolean         description: Is MM a Parent Account       isMChild:         type: boolean         description: Is MM a Child Account       isMultiplex:         type: boolean         description: Is a Multiplex Account. These are account models with individual account being parent and managed account being child.   desc:     type: string     description: Formatted \"accountId - accountAlias\"  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


