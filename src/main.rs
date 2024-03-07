#![allow(unused)]

use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

use serde::Deserialize;

#[tokio::main]
async fn main() {
    /*let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello<strong> World !!!</strong>") }),
    );
    */

    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/helloParam", get(handler_hello_param));

    let listener = TcpListener::bind("127.0.0.1:8087").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());

    println!("Listening on {:?}\n", listener);

    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    println!("TERMINAL {:<12} - handler_hello", "RESP");

    Html("Hello <string> World !!!<strong>")
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// /helloParam?name=batman`
async fn handler_hello_param(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World!");

    println!(
        "TERMINAL {:<12} - handler_hello_param - {params:?}",
        "HANDLER"
    );
    Html(format!("Hello <strong>{name}</strong>"))
}
