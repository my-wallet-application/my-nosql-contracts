service_sdk::macros::use_my_no_sql_entity!();
use serde::*;

#[my_no_sql_entity("assets")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetMyNoSqlEntity {
    #[serde(rename = "Accuracy")]
    pub accuracy: usize,
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "DepositMethods")]
    pub deposit_methods: Option<Vec<String>>,
    #[serde(rename = "IsEnabledWithdrawal")]
    pub is_enabled_withdrawal: bool,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "IconUrl")]
    pub icon_url: String,
    #[serde(rename = "IsUsd")]
    pub is_usd: Option<bool>,
}

impl AssetMyNoSqlEntity {
    pub const PARTITION_KEY: &'static str = "a";

    pub fn get_asset_id(&self) -> &str {
        &self.row_key
    }
}
