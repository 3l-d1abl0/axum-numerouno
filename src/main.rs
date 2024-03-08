#![allow(unused)]

use std::string;

use axum::extract::{Path, Query};
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::{get, get_service};
use axum::Router;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    /*let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello<strong> World !!!</strong>") }),
    );
    */

    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/helloParam", get(handler_hello_param))
        .route("/helloPath/:param", get(handler_hello_path))
        .fallback_service(routes_static());

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

///helloPath/someName
async fn handler_hello_path(Path(param): Path<String>) -> impl IntoResponse {
    println!(
        "TERMINAL {:<12} - handler_hello_path - {param:?}",
        "HANDLER"
    );
    Html(format!("Hello <strong>{param}</strong>"))
}

//static Routing
fn routes_static() -> Router {
    Router::new().nest_service(
        "/",
        get_service(
            ServeDir::new("./public/").not_found_service(ServeFile::new("public/404.html")),
        ),
    )
}
