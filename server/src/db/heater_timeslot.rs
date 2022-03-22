pub(super) async fn prepare_all(db: &mut super::DbItf) {
    db.prepare_from_file("heater_timeslot/insert").await;
}