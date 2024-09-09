service_sdk::macros::use_my_no_sql_entity!();
use serde::*;

#[my_no_sql_entity("trading-profile")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradingProfileMyNoSqlEntity {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Assets")]
    pub assets: Option<Vec<String>>,

    pub asset_pairs: Option<Vec<AssetPairProfileSettings>>,

    #[serde(rename = "ExchangeCommission")]
    pub exchange_commission: Option<f64>,
    pub default: bool,
}

impl TradingProfileMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "tp";

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetPairProfileSettings {
    pub id: String,
    pub direct: bool,
    pub reverse: bool,
}
