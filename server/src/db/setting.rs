pub(super) async fn prepare_all(db: &mut super::DbItf) {
    db.prepare_from_file("setting/select_by_key").await;
    db.prepare_from_file("setting/update_by_key").await;
}

pub async fn get_by_key(t: &super::DbTransaction<'_>, key: &str) -> String {
    t.query("setting/select_by_key", &[&key]).await.unwrap()[0].get("value")
}

pub async fn update_by_key(t: &super::DbTransaction<'_>, key: &str, value: &str) {
    t.query("setting/update_by_key", &[&key, &value]).await.unwrap();
}
