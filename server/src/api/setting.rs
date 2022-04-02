use std::collections::HashMap;
use warp::{Filter, http};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use serde_derive::{Deserialize, Serialize};
use super::db;

type Db = Arc<Mutex<db::DbItf>>;

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    key: String,
    value: String
}

pub fn create_routes(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    get_all_settings_route(db)
        .or(get_setting_route(db))
        .or(update_setting_route(db))
}

fn get_all_settings_route(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("v1" / "setting")
        .and(warp::get())
        .and(super::with_db(db.clone()))
        .and_then(get_all_settings)
}

fn get_setting_route(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("v1" / "setting" / String)
        .and(warp::get())
        .and(super::with_db(db.clone()))
        .and_then(get_setting)
}

fn update_setting_route(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("v1" / "setting")
        .and(warp::put())
        .and(setting_json_body())
        .and(super::with_db(db.clone()))
        .and_then(update_setting)
}

async fn get_all_settings(db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut dbitf = db.lock().await;
    let t = dbitf.transaction().await;
    let res = db::setting::get(&t).await;
    Ok(warp::reply::with_status(
        format!("{}", serde_json::to_string(&crate::structs::Settings { settings: res} ).unwrap()),
        http::StatusCode::OK,
    ))
}

async fn get_setting(key: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut dbitf = db.lock().await;
    let t = dbitf.transaction().await;
    let value = db::setting::get_by_key(&t, &key).await;
    Ok(warp::reply::with_status(
        format!("{}", serde_json::to_string(&Setting { key, value }).unwrap()),
        http::StatusCode::OK,
    ))
}

async fn update_setting(setting: Setting, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut dbitf = db.lock().await;
    let t = dbitf.transaction().await;
    db::setting::update_by_key(&t, &setting.key, &setting.value).await;
    t.commit().await;
    Ok(warp::reply::with_status(
        "Ok",
        http::StatusCode::OK,
    ))
}

fn setting_json_body() -> impl Filter<Extract = (Setting,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
