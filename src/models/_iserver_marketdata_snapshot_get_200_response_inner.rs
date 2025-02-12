/*
 * Client Portal Web API
 *
 * Client Poral Web API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IserverMarketdataSnapshotGet200ResponseInner {
    #[serde(rename = "server_id", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i32>,
    #[serde(rename = "_updated", skip_serializing_if = "Option::is_none")]
    pub _updated: Option<i32>,
    /// Last Price - The last price at which the contract traded. May contain one of the following prefixes:   * C - Previous day's closing price.   * H - Trading has halted. 
    #[serde(rename = "31", skip_serializing_if = "Option::is_none")]
    pub param_31: Option<String>,
    /// Symbol
    #[serde(rename = "55", skip_serializing_if = "Option::is_none")]
    pub param_55: Option<String>,
    /// Text
    #[serde(rename = "58", skip_serializing_if = "Option::is_none")]
    pub param_58: Option<String>,
    /// High - Current day high price
    #[serde(rename = "70", skip_serializing_if = "Option::is_none")]
    pub param_70: Option<String>,
    /// Low - Current day low price
    #[serde(rename = "71", skip_serializing_if = "Option::is_none")]
    pub param_71: Option<String>,
    /// Market Value - The current market value of  your position in the security. Market Value is calculated with real time market data (even when not subscribed to market data).
    #[serde(rename = "73", skip_serializing_if = "Option::is_none")]
    pub param_73: Option<String>,
    /// Avg Price - The average price of the position.
    #[serde(rename = "74", skip_serializing_if = "Option::is_none")]
    pub param_74: Option<String>,
    /// Unrealized PnL - Unrealized profit or loss. Unrealized PnL is calculated with real time market data (even when not subscribed to market data).
    #[serde(rename = "75", skip_serializing_if = "Option::is_none")]
    pub param_75: Option<String>,
    /// Formatted position
    #[serde(rename = "76", skip_serializing_if = "Option::is_none")]
    pub param_76: Option<String>,
    /// Formatted Unrealized PnL
    #[serde(rename = "77", skip_serializing_if = "Option::is_none")]
    pub param_77: Option<String>,
    /// Daily PnL - Your profit or loss of the day since prior close. Daily PnL is calculated with real time market data (even when not subscribed to market data).
    #[serde(rename = "78", skip_serializing_if = "Option::is_none")]
    pub param_78: Option<String>,
    /// Realized PnL - Realized profit or loss. Realized PnL is calculated with real time market data (even when not subscribed to market data).
    #[serde(rename = "79", skip_serializing_if = "Option::is_none")]
    pub param_79: Option<String>,
    /// Unrealized PnL % - Unrealized profit or loss expressed in percentage.
    #[serde(rename = "80", skip_serializing_if = "Option::is_none")]
    pub param_80: Option<String>,
    /// Change - The difference between the last price and the close on the previous trading day
    #[serde(rename = "82", skip_serializing_if = "Option::is_none")]
    pub param_82: Option<String>,
    /// Change % - The difference between the last price and the close on the previous trading day in percentage.
    #[serde(rename = "83", skip_serializing_if = "Option::is_none")]
    pub param_83: Option<String>,
    /// Bid Price - The highest-priced bid on the contract.
    #[serde(rename = "84", skip_serializing_if = "Option::is_none")]
    pub param_84: Option<String>,
    /// Ask Size - The number of contracts or shares offered at the ask price. For US stocks, the number displayed is divided by 100.
    #[serde(rename = "85", skip_serializing_if = "Option::is_none")]
    pub param_85: Option<String>,
    /// Ask Price - The lowest-priced offer on the contract.
    #[serde(rename = "86", skip_serializing_if = "Option::is_none")]
    pub param_86: Option<String>,
    /// Volume - Volume for the day, formatted with 'K' for thousands or 'M' for millions. For higher precision volume refer to field 7762.
    #[serde(rename = "87", skip_serializing_if = "Option::is_none")]
    pub param_87: Option<String>,
    /// Raw Volume - Volume for the day, provided in long form without formatted with K/M. This field value is deprecated, for high precision volume refer to field 7762.
    #[serde(rename = "87_raw (deprecated)", skip_serializing_if = "Option::is_none")]
    pub param_87_raw_left_parenthesis_deprecated_right_parenthesis: Option<String>,
    /// Bid Size - The number of contracts or shares bid for at the bid price. For US stocks, the number displayed is divided by 100.
    #[serde(rename = "88", skip_serializing_if = "Option::is_none")]
    pub param_88: Option<String>,
    /// Exchange
    #[serde(rename = "6004", skip_serializing_if = "Option::is_none")]
    pub param_6004: Option<String>,
    /// Conid - Contract identifier from IBKR's database.
    #[serde(rename = "6008", skip_serializing_if = "Option::is_none")]
    pub param_6008: Option<i32>,
    /// SecType - The asset class of the instrument.
    #[serde(rename = "6070", skip_serializing_if = "Option::is_none")]
    pub param_6070: Option<String>,
    /// Months
    #[serde(rename = "6072", skip_serializing_if = "Option::is_none")]
    pub param_6072: Option<String>,
    /// Regular Expiry
    #[serde(rename = "6073", skip_serializing_if = "Option::is_none")]
    pub param_6073: Option<String>,
    /// Marker for market data delivery method (similar to request id)
    #[serde(rename = "6119", skip_serializing_if = "Option::is_none")]
    pub param_6119: Option<String>,
    /// Underlying Conid. Use /trsrv/secdef to get more information about the security
    #[serde(rename = "6457", skip_serializing_if = "Option::is_none")]
    pub param_6457: Option<i32>,
    /// Service Params.
    #[serde(rename = "6508", skip_serializing_if = "Option::is_none")]
    pub param_6508: Option<String>,
    /// Market Data Availability. The field may contain three chars. First char defines: R = RealTime, D = Delayed, Z = Frozen, Y = Frozen Delayed, N = Not Subscribed. Second char defines: P = Snapshot, p = Consolidated. Third char defines: B = Book   * RealTime - Data is relayed back in real time without delay, market data subscription(s) are required.   * Delayed - Data is relayed back 15-20 min delayed.    * Frozen - Last recorded data at market close, relayed back in real time.   * Frozen Delayed - Last recorded data at market close, relayed back delayed.   * Not Subscribed - User does not have the required market data subscription(s) to relay back either real time or delayed data.   * Snapshot - Snapshot request is available for contract.   * Consolidated - Market data is aggregated across multiple exchanges or venues.   * Book - Top of the book data is available for contract. 
    #[serde(rename = "6509", skip_serializing_if = "Option::is_none")]
    pub param_6509: Option<String>,
    /// Company name
    #[serde(rename = "7051", skip_serializing_if = "Option::is_none")]
    pub param_7051: Option<String>,
    /// Ask Exch - Displays the exchange(s) offering the SMART price. A=AMEX, C=CBOE, I=ISE, X=PHLX, N=PSE, B=BOX, Q=NASDAQOM, Z=BATS, W=CBOE2, T=NASDAQBX, M=MIAX, H=GEMINI, E=EDGX, J=MERCURY 
    #[serde(rename = "7057", skip_serializing_if = "Option::is_none")]
    pub param_7057: Option<String>,
    /// Last Exch - Displays the exchange(s) offering the SMART price. A=AMEX, C=CBOE, I=ISE, X=PHLX, N=PSE, B=BOX, Q=NASDAQOM, Z=BATS, W=CBOE2, T=NASDAQBX, M=MIAX, H=GEMINI, E=EDGX, J=MERCURY 
    #[serde(rename = "7058", skip_serializing_if = "Option::is_none")]
    pub param_7058: Option<String>,
    /// Last Size - The number of unites traded at the last price
    #[serde(rename = "7059", skip_serializing_if = "Option::is_none")]
    pub param_7059: Option<String>,
    /// Bid Exch - Displays the exchange(s) offering the SMART price. A=AMEX, C=CBOE, I=ISE, X=PHLX, N=PSE, B=BOX, Q=NASDAQOM, Z=BATS, W=CBOE2, T=NASDAQBX, M=MIAX, H=GEMINI, E=EDGX, J=MERCURY 
    #[serde(rename = "7068", skip_serializing_if = "Option::is_none")]
    pub param_7068: Option<String>,
    /// Implied Vol./Hist. Vol % - The ratio of the implied volatility over the historical volatility, expressed as a percentage.
    #[serde(rename = "7084", skip_serializing_if = "Option::is_none")]
    pub param_7084: Option<String>,
    /// Put/Call Interest - Put option open interest/call option open interest for the trading day.
    #[serde(rename = "7085", skip_serializing_if = "Option::is_none")]
    pub param_7085: Option<String>,
    /// Put/Call Volume - Put option volume/call option volume for the trading day.
    #[serde(rename = "7086", skip_serializing_if = "Option::is_none")]
    pub param_7086: Option<String>,
    /// Hist. Vol. % - 30-day real-time historical volatility.
    #[serde(rename = "7087", skip_serializing_if = "Option::is_none")]
    pub param_7087: Option<String>,
    /// Hist. Vol. Close % - Shows the historical volatility based on previous close price.
    #[serde(rename = "7088", skip_serializing_if = "Option::is_none")]
    pub param_7088: Option<String>,
    /// Opt. Volume - Option Volume
    #[serde(rename = "7089", skip_serializing_if = "Option::is_none")]
    pub param_7089: Option<String>,
    /// Conid + Exchange
    #[serde(rename = "7094", skip_serializing_if = "Option::is_none")]
    pub param_7094: Option<String>,
    /// canBeTraded - If contract is a trade-able instrument. Returns 1(true) or 0(false).
    #[serde(rename = "7184", skip_serializing_if = "Option::is_none")]
    pub param_7184: Option<String>,
    /// Contract Description
    #[serde(rename = "7219", skip_serializing_if = "Option::is_none")]
    pub param_7219: Option<String>,
    /// Contract Description
    #[serde(rename = "7220", skip_serializing_if = "Option::is_none")]
    pub param_7220: Option<String>,
    /// Listing Exchange
    #[serde(rename = "7221", skip_serializing_if = "Option::is_none")]
    pub param_7221: Option<String>,
    /// Industry - Displays the type of industry under which the underlying company can be categorized.
    #[serde(rename = "7280", skip_serializing_if = "Option::is_none")]
    pub param_7280: Option<String>,
    /// Category - Displays a more detailed level of description within the industry under which the underlying company can be categorized.
    #[serde(rename = "7281", skip_serializing_if = "Option::is_none")]
    pub param_7281: Option<String>,
    /// Average Volume - The average daily trading volume over 90 days.
    #[serde(rename = "7282", skip_serializing_if = "Option::is_none")]
    pub param_7282: Option<String>,
    /// Option Implied Vol. % - A prediction of how volatile an underlying will be in the future. At the market volatility estimated for a maturity thirty calendar days forward of the current trading day, and based on option prices from two consecutive expiration months. To query the Implied Vol. % of a specific strike refer to field 7633. 
    #[serde(rename = "7283", skip_serializing_if = "Option::is_none")]
    pub param_7283: Option<String>,
    /// Historic Volume (30d)
    #[serde(rename = "7284", skip_serializing_if = "Option::is_none")]
    pub param_7284: Option<String>,
    /// Put/Call Ratio
    #[serde(rename = "7285", skip_serializing_if = "Option::is_none")]
    pub param_7285: Option<String>,
    /// Dividend Amount - Displays the amount of the next dividend.
    #[serde(rename = "7286", skip_serializing_if = "Option::is_none")]
    pub param_7286: Option<String>,
    /// Dividend Yield % - This value is the toal of the expected dividend payments over the next twelve months per share divided by the Current Price and is expressed as a percentage. For derivatives, this displays the total of the expected dividend payments over the expiry date. 
    #[serde(rename = "7287", skip_serializing_if = "Option::is_none")]
    pub param_7287: Option<String>,
    /// Ex-date of the dividend
    #[serde(rename = "7288", skip_serializing_if = "Option::is_none")]
    pub param_7288: Option<String>,
    /// Market Cap
    #[serde(rename = "7289", skip_serializing_if = "Option::is_none")]
    pub param_7289: Option<String>,
    /// P/E
    #[serde(rename = "7290", skip_serializing_if = "Option::is_none")]
    pub param_7290: Option<String>,
    /// EPS
    #[serde(rename = "7291", skip_serializing_if = "Option::is_none")]
    pub param_7291: Option<String>,
    /// Cost Basis - Your current position in this security multiplied by the average price and multiplier.
    #[serde(rename = "7292", skip_serializing_if = "Option::is_none")]
    pub param_7292: Option<String>,
    /// 52 Week High - The highest price for the past 52 weeks.
    #[serde(rename = "7293", skip_serializing_if = "Option::is_none")]
    pub param_7293: Option<String>,
    /// 52 Week Low - The lowest price for the past 52 weeks.
    #[serde(rename = "7294", skip_serializing_if = "Option::is_none")]
    pub param_7294: Option<String>,
    /// Open - Today's opening price.
    #[serde(rename = "7295", skip_serializing_if = "Option::is_none")]
    pub param_7295: Option<String>,
    /// Close - Today's closing price.
    #[serde(rename = "7296", skip_serializing_if = "Option::is_none")]
    pub param_7296: Option<String>,
    /// Delta - The ratio of the change in the price of the option to the corresponding change in the price of the underlying.
    #[serde(rename = "7308", skip_serializing_if = "Option::is_none")]
    pub param_7308: Option<String>,
    /// Gamma - The rate of change for the delta with respect to the underlying asset's price.
    #[serde(rename = "7309", skip_serializing_if = "Option::is_none")]
    pub param_7309: Option<String>,
    /// Theta - A measure of the rate of decline the value of an option due to the passage of time.
    #[serde(rename = "7310", skip_serializing_if = "Option::is_none")]
    pub param_7310: Option<String>,
    /// Vega - The amount that the price of an option changes compared to a 1% change in the volatility.
    #[serde(rename = "7311", skip_serializing_if = "Option::is_none")]
    pub param_7311: Option<String>,
    /// Opt. Volume Change % - Today's option volume as a percentage of the average option volume.
    #[serde(rename = "7607", skip_serializing_if = "Option::is_none")]
    pub param_7607: Option<String>,
    /// Implied Vol. % - The implied volatility for the specific strike of the option in percentage. To query the Option Implied Vol. % from the underlying refer to field 7283.  
    #[serde(rename = "7633", skip_serializing_if = "Option::is_none")]
    pub param_7633: Option<String>,
    /// Mark - The mark price is, the ask price if ask is less than last price, the bid price if bid is more than the last price, otherwise it's equal to last price.
    #[serde(rename = "7635", skip_serializing_if = "Option::is_none")]
    pub param_7635: Option<String>,
    /// Shortable Shares - Number of shares available for shorting.
    #[serde(rename = "7636", skip_serializing_if = "Option::is_none")]
    pub param_7636: Option<String>,
    /// Fee Rate - Interest rate charged on borrowed shares.
    #[serde(rename = "7637", skip_serializing_if = "Option::is_none")]
    pub param_7637: Option<String>,
    /// Option Open Interest
    #[serde(rename = "7638", skip_serializing_if = "Option::is_none")]
    pub param_7638: Option<String>,
    /// % of Mark Value - Displays the market value of the contract as a percentage of the total market value of the account. Mark Value is calculated with real time market data (even when not subscribed to market data). 
    #[serde(rename = "7639", skip_serializing_if = "Option::is_none")]
    pub param_7639: Option<String>,
    /// Shortable - Describes the level of difficulty with which the security can be sold short.
    #[serde(rename = "7644", skip_serializing_if = "Option::is_none")]
    pub param_7644: Option<String>,
    /// Morningstar Rating - Displays Morningstar Rating provided value. Requires [Morningstar](https://www.interactivebrokers.com/en/index.php?f=14262) subscription.
    #[serde(rename = "7655", skip_serializing_if = "Option::is_none")]
    pub param_7655: Option<String>,
    /// Dividends - This value is the total of the expected dividend payments over the next twelve months per share.
    #[serde(rename = "7671", skip_serializing_if = "Option::is_none")]
    pub param_7671: Option<String>,
    /// Dividends TTM - This value is the total of the expected dividend payments over the last twelve months per share.
    #[serde(rename = "7672", skip_serializing_if = "Option::is_none")]
    pub param_7672: Option<String>,
    /// EMA(200) - Exponential moving average (N=200).
    #[serde(rename = "7674", skip_serializing_if = "Option::is_none")]
    pub param_7674: Option<String>,
    /// EMA(100) - Exponential moving average (N=100).
    #[serde(rename = "7675", skip_serializing_if = "Option::is_none")]
    pub param_7675: Option<String>,
    /// EMA(50) - Exponential moving average (N=50).
    #[serde(rename = "7676", skip_serializing_if = "Option::is_none")]
    pub param_7676: Option<String>,
    /// EMA(20) - Exponential moving average (N=20).
    #[serde(rename = "7677", skip_serializing_if = "Option::is_none")]
    pub param_7677: Option<String>,
    /// Price/EMA(200) - Price to Exponential moving average (N=200) ratio -1, displayed in percents.
    #[serde(rename = "7678", skip_serializing_if = "Option::is_none")]
    pub param_7678: Option<String>,
    /// Price/EMA(100) - Price to Exponential moving average (N=100) ratio -1, displayed in percents.
    #[serde(rename = "7679", skip_serializing_if = "Option::is_none")]
    pub param_7679: Option<String>,
    /// Price/EMA(50) - Price to Exponential moving average (N=50) ratio -1, displayed in percents.
    #[serde(rename = "7680", skip_serializing_if = "Option::is_none")]
    pub param_7680: Option<String>,
    /// Price/EMA(20) - Price to Exponential moving average (N=20) ratio -1, displayed in percents.
    #[serde(rename = "7681", skip_serializing_if = "Option::is_none")]
    pub param_7681: Option<String>,
    /// Change Since Open - The difference between the last price and the open price.
    #[serde(rename = "7682", skip_serializing_if = "Option::is_none")]
    pub param_7682: Option<String>,
    /// Upcoming Event - Shows the next major company event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7683", skip_serializing_if = "Option::is_none")]
    pub param_7683: Option<String>,
    /// Upcoming Event Date - The date of the next major company event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7684", skip_serializing_if = "Option::is_none")]
    pub param_7684: Option<String>,
    /// Upcoming Analyst Meeting - The date and time of the next scheduled analyst meeting. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7685", skip_serializing_if = "Option::is_none")]
    pub param_7685: Option<String>,
    /// Upcoming Earnings - The date and time of the next scheduled earnings/earnings call event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7686", skip_serializing_if = "Option::is_none")]
    pub param_7686: Option<String>,
    /// Upcoming Misc Event - The date and time of the next shareholder meeting, presentation or other event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7687", skip_serializing_if = "Option::is_none")]
    pub param_7687: Option<String>,
    /// Recent Analyst Meeting - The date and time of the most recent analyst meeting. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7688", skip_serializing_if = "Option::is_none")]
    pub param_7688: Option<String>,
    /// Recent Earnings - The date and time of the most recent earnings/earning call event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7689", skip_serializing_if = "Option::is_none")]
    pub param_7689: Option<String>,
    /// Recent Misc Event - The date and time of the most recent shareholder meeting, presentation or other event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    #[serde(rename = "7690", skip_serializing_if = "Option::is_none")]
    pub param_7690: Option<String>,
    /// Probability of Max Return - Customer implied probability of maximum potential gain.
    #[serde(rename = "7694", skip_serializing_if = "Option::is_none")]
    pub param_7694: Option<String>,
    /// Break Even - Break even points
    #[serde(rename = "7695", skip_serializing_if = "Option::is_none")]
    pub param_7695: Option<String>,
    /// SPX Delta - Beta Weighted Delta is calculated using the formula; Delta x dollar adjusted beta, where adjusted beta is adjusted by the ratio of the close price.
    #[serde(rename = "7696", skip_serializing_if = "Option::is_none")]
    pub param_7696: Option<String>,
    /// Futures Open Interest - Total number of outstanding futures contracts
    #[serde(rename = "7697", skip_serializing_if = "Option::is_none")]
    pub param_7697: Option<String>,
    /// Last Yield - Implied yield of the bond if it is purchased at the current last price. Last yield is calculated using the Last price on all possible call dates. It is assumed that prepayment occurs if the bond has call or put provisions and the issuer can offer a lower coupon rate based on current market rates. The yield to worst will be the lowest of the yield to maturity or yield to call (if the bond has prepayment provisions). Yield to worse may be the same as yield to maturity but never higher. 
    #[serde(rename = "7698", skip_serializing_if = "Option::is_none")]
    pub param_7698: Option<String>,
    /// Bid Yield - Implied yield of the bond if it is purchased at the current bid price. Bid yield is calculated using the Ask on all possible call dates. It is assumed that prepayment occurs if the bond has call or put provisions and the issuer can offer a lower coupon rate based on current market rates. The yield to worst will be the lowest of the yield to maturity or yield to call (if the bond has prepayment provisions). Yield to worse may be the same as yield to maturity but never higher. 
    #[serde(rename = "7699", skip_serializing_if = "Option::is_none")]
    pub param_7699: Option<String>,
    /// Probability of Max Return - Customer implied probability of maximum potential gain.
    #[serde(rename = "7700", skip_serializing_if = "Option::is_none")]
    pub param_7700: Option<String>,
    /// Probability of Max Loss - Customer implied probability of maximum potential loss.
    #[serde(rename = "7702", skip_serializing_if = "Option::is_none")]
    pub param_7702: Option<String>,
    /// Profit Probability - Customer implied probability of any gain.
    #[serde(rename = "7703", skip_serializing_if = "Option::is_none")]
    pub param_7703: Option<String>,
    /// Organization Type
    #[serde(rename = "7704", skip_serializing_if = "Option::is_none")]
    pub param_7704: Option<String>,
    /// Debt Class
    #[serde(rename = "7705", skip_serializing_if = "Option::is_none")]
    pub param_7705: Option<String>,
    /// Ratings - Ratings issued for bond contract.
    #[serde(rename = "7706", skip_serializing_if = "Option::is_none")]
    pub param_7706: Option<String>,
    /// Bond State Code
    #[serde(rename = "7707", skip_serializing_if = "Option::is_none")]
    pub param_7707: Option<String>,
    /// Bond Type
    #[serde(rename = "7708", skip_serializing_if = "Option::is_none")]
    pub param_7708: Option<String>,
    /// Last Trading Date
    #[serde(rename = "7714", skip_serializing_if = "Option::is_none")]
    pub param_7714: Option<String>,
    /// Issue Date
    #[serde(rename = "7715", skip_serializing_if = "Option::is_none")]
    pub param_7715: Option<String>,
    /// Beta - Beta is against standard index.
    #[serde(rename = "7718", skip_serializing_if = "Option::is_none")]
    pub param_7718: Option<String>,
    /// Ask Yield - Implied yield of the bond if it is purchased at the current offer. Ask yield is calculated using the Bid on all possible call dates. It is assumed that prepayment occurs if the bond has call or put provisions and the issuer can offer a lower coupon rate based on current market rates. The yield to worst will be the lowest of the yield to maturity or yield to call (if the bond has prepayment provisions). Yield to worse may be the same as yield to maturity but never higher. 
    #[serde(rename = "7720", skip_serializing_if = "Option::is_none")]
    pub param_7720: Option<String>,
    /// Prior Close - Yesterday's closing price
    #[serde(rename = "7741", skip_serializing_if = "Option::is_none")]
    pub param_7741: Option<String>,
    /// Volume Long - High precision volume for the day. For formatted volume refer to field 87.
    #[serde(rename = "7762", skip_serializing_if = "Option::is_none")]
    pub param_7762: Option<String>,
    /// hasTradingPermissions - if user has trading permissions for specified contract. Returns 1(true) or 0(false).
    #[serde(rename = "7768", skip_serializing_if = "Option::is_none")]
    pub param_7768: Option<String>,
}

impl IserverMarketdataSnapshotGet200ResponseInner {
    pub fn new() -> IserverMarketdataSnapshotGet200ResponseInner {
        IserverMarketdataSnapshotGet200ResponseInner {
            server_id: None,
            conid: None,
            _updated: None,
            param_31: None,
            param_55: None,
            param_58: None,
            param_70: None,
            param_71: None,
            param_73: None,
            param_74: None,
            param_75: None,
            param_76: None,
            param_77: None,
            param_78: None,
            param_79: None,
            param_80: None,
            param_82: None,
            param_83: None,
            param_84: None,
            param_85: None,
            param_86: None,
            param_87: None,
            param_87_raw_left_parenthesis_deprecated_right_parenthesis: None,
            param_88: None,
            param_6004: None,
            param_6008: None,
            param_6070: None,
            param_6072: None,
            param_6073: None,
            param_6119: None,
            param_6457: None,
            param_6508: None,
            param_6509: None,
            param_7051: None,
            param_7057: None,
            param_7058: None,
            param_7059: None,
            param_7068: None,
            param_7084: None,
            param_7085: None,
            param_7086: None,
            param_7087: None,
            param_7088: None,
            param_7089: None,
            param_7094: None,
            param_7184: None,
            param_7219: None,
            param_7220: None,
            param_7221: None,
            param_7280: None,
            param_7281: None,
            param_7282: None,
            param_7283: None,
            param_7284: None,
            param_7285: None,
            param_7286: None,
            param_7287: None,
            param_7288: None,
            param_7289: None,
            param_7290: None,
            param_7291: None,
            param_7292: None,
            param_7293: None,
            param_7294: None,
            param_7295: None,
            param_7296: None,
            param_7308: None,
            param_7309: None,
            param_7310: None,
            param_7311: None,
            param_7607: None,
            param_7633: None,
            param_7635: None,
            param_7636: None,
            param_7637: None,
            param_7638: None,
            param_7639: None,
            param_7644: None,
            param_7655: None,
            param_7671: None,
            param_7672: None,
            param_7674: None,
            param_7675: None,
            param_7676: None,
            param_7677: None,
            param_7678: None,
            param_7679: None,
            param_7680: None,
            param_7681: None,
            param_7682: None,
            param_7683: None,
            param_7684: None,
            param_7685: None,
            param_7686: None,
            param_7687: None,
            param_7688: None,
            param_7689: None,
            param_7690: None,
            param_7694: None,
            param_7695: None,
            param_7696: None,
            param_7697: None,
            param_7698: None,
            param_7699: None,
            param_7700: None,
            param_7702: None,
            param_7703: None,
            param_7704: None,
            param_7705: None,
            param_7706: None,
            param_7707: None,
            param_7708: None,
            param_7714: None,
            param_7715: None,
            param_7718: None,
            param_7720: None,
            param_7741: None,
            param_7762: None,
            param_7768: None,
        }
    }
}

