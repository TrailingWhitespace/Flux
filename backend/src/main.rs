use axum::{Router, response::Html, routing::get, http::HeaderValue};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler)).layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(Any)
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Failed to start server");
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

