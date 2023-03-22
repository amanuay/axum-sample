use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server,
};
use std::sync::{Arc, Mutex};
use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/sysinfo", get(cpu_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new())),
        });
    let server = Server::bind(&"0.0.0.0:6969".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("address @port {addr}");
    server.await.unwrap();
    println!("started axum server!");
}
#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

#[axum::debug_handler]
async fn cpu_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    Json(v)
}

async fn root_get() -> impl IntoResponse {
    let html_file = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(html_file)
}
