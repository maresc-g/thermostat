mod api;
mod exec;
mod db;
mod structs;
mod hal;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting up");

    let handle_main_loop = tokio::spawn(async {
        exec::run_main_loop().await;
    });
    let handle_http_server = tokio::spawn(async {
        api::run_http_server().await;
    });
    handle_main_loop.await.unwrap();
    handle_http_server.await.unwrap();
}