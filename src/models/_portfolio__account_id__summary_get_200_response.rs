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
pub struct PortfolioAccountIdSummaryGet200Response {
    #[serde(rename = "accountready", skip_serializing_if = "Option::is_none")]
    pub accountready: Option<Box<models::Summary>>,
    #[serde(rename = "accounttype", skip_serializing_if = "Option::is_none")]
    pub accounttype: Option<Box<models::Summary>>,
    #[serde(rename = "accruedcash", skip_serializing_if = "Option::is_none")]
    pub accruedcash: Option<Box<models::Summary>>,
    #[serde(rename = "accruedcash-c", skip_serializing_if = "Option::is_none")]
    pub accruedcash_c: Option<Box<models::Summary>>,
    #[serde(rename = "accruedcash-f", skip_serializing_if = "Option::is_none")]
    pub accruedcash_f: Option<Box<models::Summary>>,
    #[serde(rename = "accruedcash-s", skip_serializing_if = "Option::is_none")]
    pub accruedcash_s: Option<Box<models::Summary>>,
    #[serde(rename = "accrueddividend", skip_serializing_if = "Option::is_none")]
    pub accrueddividend: Option<Box<models::Summary>>,
    #[serde(rename = "accrueddividend-c", skip_serializing_if = "Option::is_none")]
    pub accrueddividend_c: Option<Box<models::Summary>>,
    #[serde(rename = "accrueddividend-f", skip_serializing_if = "Option::is_none")]
    pub accrueddividend_f: Option<Box<models::Summary>>,
    #[serde(rename = "accrueddividend-s", skip_serializing_if = "Option::is_none")]
    pub accrueddividend_s: Option<Box<models::Summary>>,
    #[serde(rename = "availablefunds", skip_serializing_if = "Option::is_none")]
    pub availablefunds: Option<Box<models::Summary>>,
    #[serde(rename = "availablefunds-c", skip_serializing_if = "Option::is_none")]
    pub availablefunds_c: Option<Box<models::Summary>>,
    #[serde(rename = "availablefunds-f", skip_serializing_if = "Option::is_none")]
    pub availablefunds_f: Option<Box<models::Summary>>,
    #[serde(rename = "availablefunds-s", skip_serializing_if = "Option::is_none")]
    pub availablefunds_s: Option<Box<models::Summary>>,
    #[serde(rename = "billable", skip_serializing_if = "Option::is_none")]
    pub billable: Option<Box<models::Summary>>,
    #[serde(rename = "billable-c", skip_serializing_if = "Option::is_none")]
    pub billable_c: Option<Box<models::Summary>>,
    #[serde(rename = "billable-f", skip_serializing_if = "Option::is_none")]
    pub billable_f: Option<Box<models::Summary>>,
    #[serde(rename = "billable-s", skip_serializing_if = "Option::is_none")]
    pub billable_s: Option<Box<models::Summary>>,
    #[serde(rename = "buyingpower", skip_serializing_if = "Option::is_none")]
    pub buyingpower: Option<Box<models::Summary>>,
    #[serde(rename = "cushion", skip_serializing_if = "Option::is_none")]
    pub cushion: Option<Box<models::Summary>>,
    #[serde(rename = "daytradesremaining", skip_serializing_if = "Option::is_none")]
    pub daytradesremaining: Option<Box<models::Summary>>,
    #[serde(rename = "daytradesremainingt+1", skip_serializing_if = "Option::is_none")]
    pub daytradesremainingt_plus_1: Option<Box<models::Summary>>,
    #[serde(rename = "daytradesremainingt+2", skip_serializing_if = "Option::is_none")]
    pub daytradesremainingt_plus_2: Option<Box<models::Summary>>,
    #[serde(rename = "daytradesremainingt+3", skip_serializing_if = "Option::is_none")]
    pub daytradesremainingt_plus_3: Option<Box<models::Summary>>,
    #[serde(rename = "daytradesremainingt+4", skip_serializing_if = "Option::is_none")]
    pub daytradesremainingt_plus_4: Option<Box<models::Summary>>,
    #[serde(rename = "equitywithloanvalue", skip_serializing_if = "Option::is_none")]
    pub equitywithloanvalue: Option<Box<models::Summary>>,
    #[serde(rename = "equitywithloanvalue-c", skip_serializing_if = "Option::is_none")]
    pub equitywithloanvalue_c: Option<Box<models::Summary>>,
    #[serde(rename = "equitywithloanvalue-f", skip_serializing_if = "Option::is_none")]
    pub equitywithloanvalue_f: Option<Box<models::Summary>>,
    #[serde(rename = "equitywithloanvalue-s", skip_serializing_if = "Option::is_none")]
    pub equitywithloanvalue_s: Option<Box<models::Summary>>,
    #[serde(rename = "excessliquidity", skip_serializing_if = "Option::is_none")]
    pub excessliquidity: Option<Box<models::Summary>>,
    #[serde(rename = "excessliquidity-c", skip_serializing_if = "Option::is_none")]
    pub excessliquidity_c: Option<Box<models::Summary>>,
    #[serde(rename = "excessliquidity-f", skip_serializing_if = "Option::is_none")]
    pub excessliquidity_f: Option<Box<models::Summary>>,
    #[serde(rename = "excessliquidity-s", skip_serializing_if = "Option::is_none")]
    pub excessliquidity_s: Option<Box<models::Summary>>,
    #[serde(rename = "fullavailablefunds", skip_serializing_if = "Option::is_none")]
    pub fullavailablefunds: Option<Box<models::Summary>>,
    #[serde(rename = "fullavailablefunds-c", skip_serializing_if = "Option::is_none")]
    pub fullavailablefunds_c: Option<Box<models::Summary>>,
    #[serde(rename = "fullavailablefunds-f", skip_serializing_if = "Option::is_none")]
    pub fullavailablefunds_f: Option<Box<models::Summary>>,
    #[serde(rename = "fullavailablefunds-s", skip_serializing_if = "Option::is_none")]
    pub fullavailablefunds_s: Option<Box<models::Summary>>,
    #[serde(rename = "fullexcessliquidity", skip_serializing_if = "Option::is_none")]
    pub fullexcessliquidity: Option<Box<models::Summary>>,
    #[serde(rename = "fullexcessliquidity-c", skip_serializing_if = "Option::is_none")]
    pub fullexcessliquidity_c: Option<Box<models::Summary>>,
    #[serde(rename = "fullexcessliquidity-f", skip_serializing_if = "Option::is_none")]
    pub fullexcessliquidity_f: Option<Box<models::Summary>>,
    #[serde(rename = "fullexcessliquidity-s", skip_serializing_if = "Option::is_none")]
    pub fullexcessliquidity_s: Option<Box<models::Summary>>,
    #[serde(rename = "fullinitmarginreq", skip_serializing_if = "Option::is_none")]
    pub fullinitmarginreq: Option<Box<models::Summary>>,
    #[serde(rename = "fullinitmarginreq-c", skip_serializing_if = "Option::is_none")]
    pub fullinitmarginreq_c: Option<Box<models::Summary>>,
    #[serde(rename = "fullinitmarginreq-f", skip_serializing_if = "Option::is_none")]
    pub fullinitmarginreq_f: Option<Box<models::Summary>>,
    #[serde(rename = "fullinitmarginreq-s", skip_serializing_if = "Option::is_none")]
    pub fullinitmarginreq_s: Option<Box<models::Summary>>,
    #[serde(rename = "fullmaintmarginreq", skip_serializing_if = "Option::is_none")]
    pub fullmaintmarginreq: Option<Box<models::Summary>>,
    #[serde(rename = "fullmaintmarginreq-c", skip_serializing_if = "Option::is_none")]
    pub fullmaintmarginreq_c: Option<Box<models::Summary>>,
    #[serde(rename = "fullmaintmarginreq-f", skip_serializing_if = "Option::is_none")]
    pub fullmaintmarginreq_f: Option<Box<models::Summary>>,
    #[serde(rename = "fullmaintmarginreq-s", skip_serializing_if = "Option::is_none")]
    pub fullmaintmarginreq_s: Option<Box<models::Summary>>,
    #[serde(rename = "grosspositionvalue", skip_serializing_if = "Option::is_none")]
    pub grosspositionvalue: Option<Box<models::Summary>>,
    #[serde(rename = "grosspositionvalue-c", skip_serializing_if = "Option::is_none")]
    pub grosspositionvalue_c: Option<Box<models::Summary>>,
    #[serde(rename = "grosspositionvalue-f", skip_serializing_if = "Option::is_none")]
    pub grosspositionvalue_f: Option<Box<models::Summary>>,
    #[serde(rename = "grosspositionvalue-s", skip_serializing_if = "Option::is_none")]
    pub grosspositionvalue_s: Option<Box<models::Summary>>,
    #[serde(rename = "guarantee", skip_serializing_if = "Option::is_none")]
    pub guarantee: Option<Box<models::Summary>>,
    #[serde(rename = "guarantee-c", skip_serializing_if = "Option::is_none")]
    pub guarantee_c: Option<Box<models::Summary>>,
    #[serde(rename = "guarantee-f", skip_serializing_if = "Option::is_none")]
    pub guarantee_f: Option<Box<models::Summary>>,
    #[serde(rename = "guarantee-s", skip_serializing_if = "Option::is_none")]
    pub guarantee_s: Option<Box<models::Summary>>,
    #[serde(rename = "highestseverity", skip_serializing_if = "Option::is_none")]
    pub highestseverity: Option<Box<models::Summary>>,
    #[serde(rename = "highestseverity-c", skip_serializing_if = "Option::is_none")]
    pub highestseverity_c: Option<Box<models::Summary>>,
    #[serde(rename = "highestseverity-f", skip_serializing_if = "Option::is_none")]
    pub highestseverity_f: Option<Box<models::Summary>>,
    #[serde(rename = "highestseverity-s", skip_serializing_if = "Option::is_none")]
    pub highestseverity_s: Option<Box<models::Summary>>,
    #[serde(rename = "indianstockhaircut", skip_serializing_if = "Option::is_none")]
    pub indianstockhaircut: Option<Box<models::Summary>>,
    #[serde(rename = "indianstockhaircut-c", skip_serializing_if = "Option::is_none")]
    pub indianstockhaircut_c: Option<Box<models::Summary>>,
    #[serde(rename = "indianstockhaircut-f", skip_serializing_if = "Option::is_none")]
    pub indianstockhaircut_f: Option<Box<models::Summary>>,
    #[serde(rename = "indianstockhaircut-s", skip_serializing_if = "Option::is_none")]
    pub indianstockhaircut_s: Option<Box<models::Summary>>,
    #[serde(rename = "initmarginreq", skip_serializing_if = "Option::is_none")]
    pub initmarginreq: Option<Box<models::Summary>>,
    #[serde(rename = "initmarginreq-c", skip_serializing_if = "Option::is_none")]
    pub initmarginreq_c: Option<Box<models::Summary>>,
    #[serde(rename = "initmarginreq-f", skip_serializing_if = "Option::is_none")]
    pub initmarginreq_f: Option<Box<models::Summary>>,
    #[serde(rename = "initmarginreq-s", skip_serializing_if = "Option::is_none")]
    pub initmarginreq_s: Option<Box<models::Summary>>,
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<Box<models::Summary>>,
    #[serde(rename = "leverage-c", skip_serializing_if = "Option::is_none")]
    pub leverage_c: Option<Box<models::Summary>>,
    #[serde(rename = "leverage-f", skip_serializing_if = "Option::is_none")]
    pub leverage_f: Option<Box<models::Summary>>,
    #[serde(rename = "leverage-s", skip_serializing_if = "Option::is_none")]
    pub leverage_s: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadavailablefunds", skip_serializing_if = "Option::is_none")]
    pub lookaheadavailablefunds: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadavailablefunds-c", skip_serializing_if = "Option::is_none")]
    pub lookaheadavailablefunds_c: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadavailablefunds-f", skip_serializing_if = "Option::is_none")]
    pub lookaheadavailablefunds_f: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadavailablefunds-s", skip_serializing_if = "Option::is_none")]
    pub lookaheadavailablefunds_s: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadexcessliquidity", skip_serializing_if = "Option::is_none")]
    pub lookaheadexcessliquidity: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadexcessliquidity-c", skip_serializing_if = "Option::is_none")]
    pub lookaheadexcessliquidity_c: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadexcessliquidity-f", skip_serializing_if = "Option::is_none")]
    pub lookaheadexcessliquidity_f: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadexcessliquidity-s", skip_serializing_if = "Option::is_none")]
    pub lookaheadexcessliquidity_s: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadinitmarginreq", skip_serializing_if = "Option::is_none")]
    pub lookaheadinitmarginreq: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadinitmarginreq-c", skip_serializing_if = "Option::is_none")]
    pub lookaheadinitmarginreq_c: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadinitmarginreq-f", skip_serializing_if = "Option::is_none")]
    pub lookaheadinitmarginreq_f: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadinitmarginreq-s", skip_serializing_if = "Option::is_none")]
    pub lookaheadinitmarginreq_s: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadmaintmarginreq", skip_serializing_if = "Option::is_none")]
    pub lookaheadmaintmarginreq: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadmaintmarginreq-c", skip_serializing_if = "Option::is_none")]
    pub lookaheadmaintmarginreq_c: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadmaintmarginreq-f", skip_serializing_if = "Option::is_none")]
    pub lookaheadmaintmarginreq_f: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadmaintmarginreq-s", skip_serializing_if = "Option::is_none")]
    pub lookaheadmaintmarginreq_s: Option<Box<models::Summary>>,
    #[serde(rename = "lookaheadnextchange", skip_serializing_if = "Option::is_none")]
    pub lookaheadnextchange: Option<Box<models::Summary>>,
    #[serde(rename = "maintmarginreq", skip_serializing_if = "Option::is_none")]
    pub maintmarginreq: Option<Box<models::Summary>>,
    #[serde(rename = "maintmarginreq-c", skip_serializing_if = "Option::is_none")]
    pub maintmarginreq_c: Option<Box<models::Summary>>,
    #[serde(rename = "maintmarginreq-f", skip_serializing_if = "Option::is_none")]
    pub maintmarginreq_f: Option<Box<models::Summary>>,
    #[serde(rename = "maintmarginreq-s", skip_serializing_if = "Option::is_none")]
    pub maintmarginreq_s: Option<Box<models::Summary>>,
    #[serde(rename = "netliquidation", skip_serializing_if = "Option::is_none")]
    pub netliquidation: Option<Box<models::Summary>>,
    #[serde(rename = "netliquidation-c", skip_serializing_if = "Option::is_none")]
    pub netliquidation_c: Option<Box<models::Summary>>,
    #[serde(rename = "netliquidation-f", skip_serializing_if = "Option::is_none")]
    pub netliquidation_f: Option<Box<models::Summary>>,
    #[serde(rename = "netliquidation-s", skip_serializing_if = "Option::is_none")]
    pub netliquidation_s: Option<Box<models::Summary>>,
    #[serde(rename = "netliquidationuncertainty", skip_serializing_if = "Option::is_none")]
    pub netliquidationuncertainty: Option<Box<models::Summary>>,
    #[serde(rename = "nlvandmargininreview", skip_serializing_if = "Option::is_none")]
    pub nlvandmargininreview: Option<Box<models::Summary>>,
    #[serde(rename = "pasharesvalue", skip_serializing_if = "Option::is_none")]
    pub pasharesvalue: Option<Box<models::Summary>>,
    #[serde(rename = "pasharesvalue-c", skip_serializing_if = "Option::is_none")]
    pub pasharesvalue_c: Option<Box<models::Summary>>,
    #[serde(rename = "pasharesvalue-f", skip_serializing_if = "Option::is_none")]
    pub pasharesvalue_f: Option<Box<models::Summary>>,
    #[serde(rename = "pasharesvalue-s", skip_serializing_if = "Option::is_none")]
    pub pasharesvalue_s: Option<Box<models::Summary>>,
    #[serde(rename = "postexpirationexcess", skip_serializing_if = "Option::is_none")]
    pub postexpirationexcess: Option<Box<models::Summary>>,
    #[serde(rename = "postexpirationexcess-c", skip_serializing_if = "Option::is_none")]
    pub postexpirationexcess_c: Option<Box<models::Summary>>,
    #[serde(rename = "postexpirationexcess-f", skip_serializing_if = "Option::is_none")]
    pub postexpirationexcess_f: Option<Box<models::Summary>>,
    #[serde(rename = "postexpirationexcess-s", skip_serializing_if = "Option::is_none")]
    pub postexpirationexcess_s: Option<Box<models::Summary>>,
    #[serde(rename = "postexpirationmargin", skip_serializing_if = "Option::is_none")]
    pub postexpirationmargin: Option<Box<models::Summary>>,
    #[serde(rename = "postexpirationmargin-c", skip_serializing_if = "Option::is_none")]
    pub postexpirationmargin_c: Option<Box<models::Summary>>,
    #[serde(rename = "postexpirationmargin-f", skip_serializing_if = "Option::is_none")]
    pub postexpirationmargin_f: Option<Box<models::Summary>>,
    #[serde(rename = "postexpirationmargin-s", skip_serializing_if = "Option::is_none")]
    pub postexpirationmargin_s: Option<Box<models::Summary>>,
    #[serde(rename = "previousdayequitywithloanvalue", skip_serializing_if = "Option::is_none")]
    pub previousdayequitywithloanvalue: Option<Box<models::Summary>>,
    #[serde(rename = "previousdayequitywithloanvalue-c", skip_serializing_if = "Option::is_none")]
    pub previousdayequitywithloanvalue_c: Option<Box<models::Summary>>,
    #[serde(rename = "previousdayequitywithloanvalue-f", skip_serializing_if = "Option::is_none")]
    pub previousdayequitywithloanvalue_f: Option<Box<models::Summary>>,
    #[serde(rename = "previousdayequitywithloanvalue-s", skip_serializing_if = "Option::is_none")]
    pub previousdayequitywithloanvalue_s: Option<Box<models::Summary>>,
    #[serde(rename = "segmenttitle-c", skip_serializing_if = "Option::is_none")]
    pub segmenttitle_c: Option<Box<models::Summary>>,
    #[serde(rename = "segmenttitle-f", skip_serializing_if = "Option::is_none")]
    pub segmenttitle_f: Option<Box<models::Summary>>,
    #[serde(rename = "segmenttitle-s", skip_serializing_if = "Option::is_none")]
    pub segmenttitle_s: Option<Box<models::Summary>>,
    #[serde(rename = "totalcashvalue", skip_serializing_if = "Option::is_none")]
    pub totalcashvalue: Option<Box<models::Summary>>,
    #[serde(rename = "totalcashvalue-c", skip_serializing_if = "Option::is_none")]
    pub totalcashvalue_c: Option<Box<models::Summary>>,
    #[serde(rename = "totalcashvalue-f", skip_serializing_if = "Option::is_none")]
    pub totalcashvalue_f: Option<Box<models::Summary>>,
    #[serde(rename = "totalcashvalue-s", skip_serializing_if = "Option::is_none")]
    pub totalcashvalue_s: Option<Box<models::Summary>>,
    #[serde(rename = "totaldebitcardpendingcharges", skip_serializing_if = "Option::is_none")]
    pub totaldebitcardpendingcharges: Option<Box<models::Summary>>,
    #[serde(rename = "totaldebitcardpendingcharges-c", skip_serializing_if = "Option::is_none")]
    pub totaldebitcardpendingcharges_c: Option<Box<models::Summary>>,
    #[serde(rename = "totaldebitcardpendingcharges-f", skip_serializing_if = "Option::is_none")]
    pub totaldebitcardpendingcharges_f: Option<Box<models::Summary>>,
    #[serde(rename = "totaldebitcardpendingcharges-s", skip_serializing_if = "Option::is_none")]
    pub totaldebitcardpendingcharges_s: Option<Box<models::Summary>>,
    #[serde(rename = "tradingtype-f", skip_serializing_if = "Option::is_none")]
    pub tradingtype_f: Option<Box<models::Summary>>,
    #[serde(rename = "tradingtype-s", skip_serializing_if = "Option::is_none")]
    pub tradingtype_s: Option<Box<models::Summary>>,
}

impl PortfolioAccountIdSummaryGet200Response {
    pub fn new() -> PortfolioAccountIdSummaryGet200Response {
        PortfolioAccountIdSummaryGet200Response {
            accountready: None,
            accounttype: None,
            accruedcash: None,
            accruedcash_c: None,
            accruedcash_f: None,
            accruedcash_s: None,
            accrueddividend: None,
            accrueddividend_c: None,
            accrueddividend_f: None,
            accrueddividend_s: None,
            availablefunds: None,
            availablefunds_c: None,
            availablefunds_f: None,
            availablefunds_s: None,
            billable: None,
            billable_c: None,
            billable_f: None,
            billable_s: None,
            buyingpower: None,
            cushion: None,
            daytradesremaining: None,
            daytradesremainingt_plus_1: None,
            daytradesremainingt_plus_2: None,
            daytradesremainingt_plus_3: None,
            daytradesremainingt_plus_4: None,
            equitywithloanvalue: None,
            equitywithloanvalue_c: None,
            equitywithloanvalue_f: None,
            equitywithloanvalue_s: None,
            excessliquidity: None,
            excessliquidity_c: None,
            excessliquidity_f: None,
            excessliquidity_s: None,
            fullavailablefunds: None,
            fullavailablefunds_c: None,
            fullavailablefunds_f: None,
            fullavailablefunds_s: None,
            fullexcessliquidity: None,
            fullexcessliquidity_c: None,
            fullexcessliquidity_f: None,
            fullexcessliquidity_s: None,
            fullinitmarginreq: None,
            fullinitmarginreq_c: None,
            fullinitmarginreq_f: None,
            fullinitmarginreq_s: None,
            fullmaintmarginreq: None,
            fullmaintmarginreq_c: None,
            fullmaintmarginreq_f: None,
            fullmaintmarginreq_s: None,
            grosspositionvalue: None,
            grosspositionvalue_c: None,
            grosspositionvalue_f: None,
            grosspositionvalue_s: None,
            guarantee: None,
            guarantee_c: None,
            guarantee_f: None,
            guarantee_s: None,
            highestseverity: None,
            highestseverity_c: None,
            highestseverity_f: None,
            highestseverity_s: None,
            indianstockhaircut: None,
            indianstockhaircut_c: None,
            indianstockhaircut_f: None,
            indianstockhaircut_s: None,
            initmarginreq: None,
            initmarginreq_c: None,
            initmarginreq_f: None,
            initmarginreq_s: None,
            leverage: None,
            leverage_c: None,
            leverage_f: None,
            leverage_s: None,
            lookaheadavailablefunds: None,
            lookaheadavailablefunds_c: None,
            lookaheadavailablefunds_f: None,
            lookaheadavailablefunds_s: None,
            lookaheadexcessliquidity: None,
            lookaheadexcessliquidity_c: None,
            lookaheadexcessliquidity_f: None,
            lookaheadexcessliquidity_s: None,
            lookaheadinitmarginreq: None,
            lookaheadinitmarginreq_c: None,
            lookaheadinitmarginreq_f: None,
            lookaheadinitmarginreq_s: None,
            lookaheadmaintmarginreq: None,
            lookaheadmaintmarginreq_c: None,
            lookaheadmaintmarginreq_f: None,
            lookaheadmaintmarginreq_s: None,
            lookaheadnextchange: None,
            maintmarginreq: None,
            maintmarginreq_c: None,
            maintmarginreq_f: None,
            maintmarginreq_s: None,
            netliquidation: None,
            netliquidation_c: None,
            netliquidation_f: None,
            netliquidation_s: None,
            netliquidationuncertainty: None,
            nlvandmargininreview: None,
            pasharesvalue: None,
            pasharesvalue_c: None,
            pasharesvalue_f: None,
            pasharesvalue_s: None,
            postexpirationexcess: None,
            postexpirationexcess_c: None,
            postexpirationexcess_f: None,
            postexpirationexcess_s: None,
            postexpirationmargin: None,
            postexpirationmargin_c: None,
            postexpirationmargin_f: None,
            postexpirationmargin_s: None,
            previousdayequitywithloanvalue: None,
            previousdayequitywithloanvalue_c: None,
            previousdayequitywithloanvalue_f: None,
            previousdayequitywithloanvalue_s: None,
            segmenttitle_c: None,
            segmenttitle_f: None,
            segmenttitle_s: None,
            totalcashvalue: None,
            totalcashvalue_c: None,
            totalcashvalue_f: None,
            totalcashvalue_s: None,
            totaldebitcardpendingcharges: None,
            totaldebitcardpendingcharges_c: None,
            totaldebitcardpendingcharges_f: None,
            totaldebitcardpendingcharges_s: None,
            tradingtype_f: None,
            tradingtype_s: None,
        }
    }
}

