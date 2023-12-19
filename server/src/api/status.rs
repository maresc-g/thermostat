use warp::{Filter, http};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use super::db;

type Db = Arc<Mutex<db::DbItf>>;

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    default_temperature: f64,
    holiday_mode_enabled: bool,
    manual_mode_enabled: bool,
    manual_mode_temperature: f64,
    is_heating: bool,
    hysteresis: f64,
    current_temperature: f64,
    target_temperature: f64,
}

pub fn create_routes(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    get_status_route(db)
}

fn get_status_route(db: &Db) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::path!("v1" / "status")
        .and(warp::get())
        .and(super::with_db(db.clone()))
        .and_then(get_status)
}

async fn get_status(db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut dbitf = db.lock().await;
    let t = dbitf.transaction().await;
    let res = db::setting::get(&t).await;
    let a: f64 = FromStr::from_str(&res["default_temperature"]).unwrap();
    println!("{:?}", a);
    let status = Status {
        default_temperature: FromStr::from_str(&res["default_temperature"]).unwrap(),
        holiday_mode_enabled: FromStr::from_str(&res["holiday_mode_enabled"]).unwrap(),
        manual_mode_enabled: FromStr::from_str(&res["manual_mode_enabled"]).unwrap(),
        manual_mode_temperature: FromStr::from_str(&res["manual_mode_temperature"]).unwrap(),
        is_heating: FromStr::from_str(&res["is_heating"]).unwrap(),
        hysteresis: FromStr::from_str(&res["hysteresis"]).unwrap(),
        current_temperature: FromStr::from_str(&res["current_temperature"]).unwrap(),
        target_temperature: FromStr::from_str(&res["target_temperature"]).unwrap(),
    };
    Ok(warp::reply::with_status(
        format!("{}", serde_json::to_string(&status).unwrap()),
        http::StatusCode::OK,
    ))
}

fn setting_json_body() -> impl Filter<Extract = (Status,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
