use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root_get));
    let server = Server::bind(&"0.0.0.0:6969".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("address {addr}");
    server.await.unwrap();
    println!("Hello, world!");
}

async fn root_get() -> &'static str {
    "Hi from axum!"
}
