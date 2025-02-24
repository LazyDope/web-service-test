use axum::{extract::State, routing, Router};
use std::sync::{Arc, Mutex};
use sysinfo::System;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/api/cpus", routing::get(cpu_get))
        .route("/", routing::get(root_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new_all())),
        });
    let listener = TcpListener::bind("0.0.0.0:7040").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn root_get() -> &'static str {
    "Hello World"
}

async fn cpu_get(State(state): State<AppState>) -> String {
    let sys = state.sys.lock().unwrap();
    use std::fmt::Write;
    let mut buf = String::new();
    for (i, cpu) in sys.cpus().iter().enumerate() {
        let _ = writeln!(buf, "Cpu {}, {}", i, cpu.cpu_usage());
    }
    buf
}

impl AppState {
    async fn sys_update(&mut self) {
        self.sys.lock().unwrap().refresh_all();
    }
}
