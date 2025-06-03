service_sdk::macros::use_my_no_sql_entity!();
use serde::*;

#[my_no_sql_entity("asset-markup")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetMarkupProfileMyNoSqlEntity {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "BidMarkup")]
    pub bid_markup: i32,
    #[serde(rename = "AskMarkup")]
    pub ask_markup: i32,
}

impl AssetMarkupProfileMyNoSqlEntity {
    pub fn get_markup_profile_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_asset_id(&self) -> &str {
        &self.row_key
    }
}
