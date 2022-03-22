use chrono::{DateTime, FixedOffset};
use serde_derive::{Deserialize, Serialize};
use tokio_postgres::row::Row;

#[derive(Deserialize, Serialize, Debug)]
pub struct TemperatureHistory {
    temperature: f64,
    date: DateTime<FixedOffset>
}

pub(super) async fn prepare_all(db: &mut super::DbItf) {
    db.prepare_from_file("temperature/select_by_min_max").await;
    db.prepare_from_file("temperature/insert").await;
}

pub fn to_temperature_history(row: Row) -> TemperatureHistory {
    TemperatureHistory {
        temperature: row.get("temperature"),
        date: row.get("date")
    }
}

pub fn to_temperature_history_vec(rows: Vec<Row>) -> Vec<TemperatureHistory> {
    let mut res = Vec::new();
    for r in rows {
        res.push(to_temperature_history(r));
    }
    res
}