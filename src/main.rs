use axum::{routing::get, Router, Server};
use std::fmt::Write;
use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root_get));
    let server = Server::bind(&"0.0.0.0:6969".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("address {addr}");
    server.await.unwrap();
    println!("Hello, world!");
}

async fn root_get() -> String {
    let mut s = String::new();
    let mut sys = System::new();
    sys.refresh_cpu();
    for (i, cpu) in sys.cpus().iter().enumerate() {
        let i = i + 1;
        let usage = cpu.cpu_usage();
        writeln!(&mut s, "CPU {i} {usage}%").unwrap();
    }
    s
}
