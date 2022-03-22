use crate::structs::HeaterTimeSlot;

pub(super) async fn prepare_all(db: &mut super::DbItf) {
    db.prepare_from_file("heater_timeslot/insert").await;
}

pub async fn insert(db: &super::DbItf, ts: &HeaterTimeSlot) {
    db.query("heater_timeslot/insert", &[&ts.target_temperature, &ts.start_day, &ts.start_time, &ts.end_day, &ts.end_time]).await.unwrap();
}
