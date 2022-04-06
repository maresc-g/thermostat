use chrono::NaiveTime;
use tokio_postgres::Row;
use crate::structs::HeaterTimeSlot;

pub(super) async fn prepare_all(db: &mut super::DbItf) {
    db.prepare_from_file("heater_timeslot/select").await;
    db.prepare_from_file("heater_timeslot/select_by_date").await;
    db.prepare_from_file("heater_timeslot/insert").await;
    db.prepare_from_file("heater_timeslot/update").await;
    db.prepare_from_file("heater_timeslot/delete").await;
}

pub async fn get(t: &super::DbTransaction<'_>) -> Vec<HeaterTimeSlot> {
    to_heater_timeslot_vec(t.query("heater_timeslot/select", &[]).await.unwrap())
}

pub async fn get_current_timeslot(t: &super::DbTransaction<'_>, day: &u32, time: &NaiveTime) -> Option<HeaterTimeSlot> {
    let rows = t.query("heater_timeslot/select_by_date", &[&(*day as i32), time]).await.unwrap();
    if rows.is_empty() {
        return Option::None;
    }
    Option::Some(to_heater_timeslot(&rows[0]))
}

pub async fn insert(t: &super::DbTransaction<'_>, ts: &HeaterTimeSlot) {
    t.query("heater_timeslot/insert", &[&ts.target_temperature, &ts.day, &ts.start_time, &ts.end_time]).await.unwrap();
}

pub async fn update(t: &super::DbTransaction<'_>, ts: &HeaterTimeSlot) {
    t.query("heater_timeslot/update", &[&ts.pk, &ts.target_temperature, &ts.day, &ts.start_time, &ts.end_time]).await.unwrap();
}

pub async fn delete(t: &super::DbTransaction<'_>, pk: &i64) {
    t.query("heater_timeslot/delete", &[&pk]).await.unwrap();
}

fn to_heater_timeslot(row: &Row) -> HeaterTimeSlot {
    HeaterTimeSlot {
        pk: row.get("pk"),
        target_temperature: row.get("target_temperature"),
        day: row.get("day"),
        start_time: row.get("start_time"),
        end_time: row.get("end_time"),
    }
}

fn to_heater_timeslot_vec(rows: Vec<Row>) -> Vec<HeaterTimeSlot> {
    let mut res = Vec::new();
    for r in rows {
        res.push(to_heater_timeslot(&r));
    }
    res
}