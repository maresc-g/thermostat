use warp::{Filter, http};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use std::ops::Deref;
use serde_derive::{Deserialize, Serialize};
use super::db;

type Db = Arc<Mutex<db::DbItf>>;

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    key: String,
    value: String
}

pub fn create_routes(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    get_setting_route(db)
        .or(update_setting_route(db))
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

async fn get_setting(key: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let value = db::setting::get_by_key(db.lock().await.deref(), &key).await;
    Ok(warp::reply::with_status(
        format!("{}", serde_json::to_string(&Setting { key, value }).unwrap()),
        http::StatusCode::OK,
    ))
}

async fn update_setting(setting: Setting, db: Db) -> Result<impl warp::Reply, Infallible> {
    db::setting::update_by_key(db.lock().await.deref(), &setting.key, &setting.value).await;
    Ok(warp::reply::with_status(
        "Ok",
        http::StatusCode::OK,
    ))
}

fn setting_json_body() -> impl Filter<Extract = (Setting,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
