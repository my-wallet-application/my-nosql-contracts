use std::collections::BTreeMap;

use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("mini-graph")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MiniGraphNoSqlEntity {
    pub data: Vec<f64>,
    pub candles: Option<BTreeMap<u32, [f64; 4]>>, //hour_key, open, high, low, close
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

    pub fn update_candle(&mut self, hour_key: u32, price: f64) {
        if self.candles.is_none() {
            self.candles = Some(BTreeMap::new());
        }

        let candles = self.candles.as_mut().unwrap();
        if let Some(value) = candles.get_mut(&hour_key) {
            if price > value[1] {
                value[1] = price;
            }

            if price < value[2] {
                value[2] = price;
            }

            value[3] = price;
        } else {
            candles.insert(hour_key, [price, price, price, price]);
        }

        if candles.len() > 80 {
            candles.pop_first();
        }
    }
}
