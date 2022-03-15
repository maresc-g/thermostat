mod api;
mod exec;
mod db;

#[tokio::main]
async fn main() {

    println!("1");
    let dbitf = db::DbItf::new();
    // dbitf.query();
    let a: i32 = dbitf.await.query("a", &[]).await.unwrap()[0].get(0);
    println!("aaaa = {}", a);
    let handle_main_loop = tokio::spawn(async {
        exec::run_main_loop().await;
    });
    let handle_http_server = tokio::spawn(async {
        api::run_http_server().await;
    });
    handle_main_loop.await.unwrap();
    handle_http_server.await.unwrap();
}