use std::net::SocketAddr;

use warp::Filter;

mod bootstrap;
mod server;
mod libs;

#[tokio::main]
async fn main() {
    bootstrap::init()
        .expect("Failed to initialize the application");

    let addr: SocketAddr = "0.0.0.0:3030".parse().unwrap();

    let api = server::routers::router();
    let routes = api.with(warp::log("rust-avatar-proxy"));
    warp::serve(routes)
        .run(addr)
        .await;
}
