service_sdk::macros::use_my_no_sql_entity!();
use serde::*;

#[my_no_sql_entity("trading-conditions")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradingConditionsProfile {
    #[serde(rename = "DirectExchange")]
    pub direct_exchange: bool,
    #[serde(rename = "ReverseExchange")]
    pub reverse_exchange: bool,
    #[serde(rename = "DirectExchangeCommission")]
    pub direct_exchange_commission: f64, //100 means 100%
    #[serde(rename = "ReverseExchangeCommission")]
    pub reverse_exchange_commission: f64, //100 means 100%
    #[serde(rename = "BidMarkup")]
    pub bid_markup: i32,
    #[serde(rename = "AskMarkup")]
    pub ask_markup: i32,
}

impl TradingConditionsProfile {
    pub fn get_trading_group_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_asset_id(&self) -> &str {
        &self.row_key
    }
}
