use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("mini-graph")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MiniGraphNoSqlEntity {
    pub data: Vec<f64>,
}

impl MiniGraphNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "m"
    }

    pub fn generate_row_key(instrument_id: &'static str) -> &'static str {
        instrument_id
    }

    pub fn get_instrument_id(&self) -> &str {
        &self.row_key
    }

    pub fn insert_value(&mut self, value: f64) {
        self.data.push(value);
        if self.data.len() > 240 {
            self.data.remove(0);
        }
    }
}
