use tokio_postgres::Row;
use crate::structs::HeaterTimeSlot;

pub(super) async fn prepare_all(db: &mut super::DbItf) {
    db.prepare_from_file("heater_timeslot/select").await;
    db.prepare_from_file("heater_timeslot/insert").await;
    db.prepare_from_file("heater_timeslot/update").await;
    db.prepare_from_file("heater_timeslot/delete").await;
}

pub async fn get(db: &super::DbItf) -> Vec<HeaterTimeSlot> {
    to_heater_timeslot_vec(db.query("heater_timeslot/select", &[]).await.unwrap())
}

pub async fn insert(db: &super::DbItf, ts: &HeaterTimeSlot) {
    db.query("heater_timeslot/insert", &[&ts.target_temperature, &ts.start_day, &ts.start_time, &ts.end_day, &ts.end_time]).await.unwrap();
}

pub async fn update(db: &super::DbItf, ts: &HeaterTimeSlot) {
    db.query("heater_timeslot/update", &[&ts.pk, &ts.target_temperature, &ts.start_day, &ts.start_time, &ts.end_day, &ts.end_time]).await.unwrap();
}

pub async fn delete(db: &super::DbItf, pk: &i64) {
    db.query("heater_timeslot/delete", &[&pk]).await.unwrap();
}

fn to_heater_timeslot(row: Row) -> HeaterTimeSlot {
    HeaterTimeSlot {
        pk: row.get("pk"),
        target_temperature: row.get("target_temperature"),
        start_day: row.get("start_day"),
        start_time: row.get("start_time"),
        end_day: row.get("end_day"),
        end_time: row.get("end_time"),
    }
}

fn to_heater_timeslot_vec(rows: Vec<Row>) -> Vec<HeaterTimeSlot> {
    let mut res = Vec::new();
    for r in rows {
        res.push(to_heater_timeslot(r));
    }
    res
}