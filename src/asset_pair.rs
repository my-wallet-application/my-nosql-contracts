service_sdk::macros::use_my_no_sql_entity!();
use serde::*;

#[my_no_sql_entity("asset-pairs")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetPairMyNoSqlEntity {
    #[serde(rename = "FromAsset")]
    pub from_asset: String,
    #[serde(rename = "ToAsset")]
    pub to_asset: String,
    #[serde(rename = "PriceAccuracy")]
    pub price_accuracy: usize,
}

impl AssetPairMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "ap";

    pub fn get_id(&self) -> &str {
        &self.row_key
    }
}
