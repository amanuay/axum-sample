use axum::{extract::State, routing::get, Router, Server};
use std::{
    fmt::Write,
    sync::{Arc, Mutex},
};
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
    println!("address {addr}");
    server.await.unwrap();
    println!("Hello, world!");
}
#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn cpu_get(State(state): State<AppState>) -> String {
    let mut s = String::new();
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    for (i, cpu) in sys.cpus().iter().enumerate() {
        let i = i + 1;
        let usage = cpu.cpu_usage();
        writeln!(&mut s, "CPU {i} {usage}%").unwrap();
    }
    s
}

async fn root_get() -> &'static str {
    "hello axum!"
}
