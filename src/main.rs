use axum::{Router, routing::get};
use tokio::net::TcpListener;

mod config;

const PORT: u16 = 3000;
const HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    let router: Router = Router::new().route("/", get(|| async { "Hello world" }));

    let addr: String = format!("{}:{}", HOST, PORT);

    let listener: TcpListener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, router).await.unwrap()
}
