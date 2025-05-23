service_sdk::macros::use_my_no_sql_entity!();
use serde::*;

#[my_no_sql_entity("mt-instrument")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyInstrument {
    pub name: String,
    pub from_asset: String,
    pub to_asset: String,
}

impl MyInstrument {
    pub fn get_group_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_instrument_id(&self) -> &str {
        &self.row_key
    }
}
