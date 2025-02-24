use axum::{routing, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", routing::get(root_get));
    let listener = TcpListener::bind("0.0.0.0:7040").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root_get() -> String {
    String::from("Hello world!")
}
