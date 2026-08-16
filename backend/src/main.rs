use axum::{
    Json, Router,
    http::HeaderValue,
    response::{Html, IntoResponse},
    routing::get,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(check_health))
        .route("/todos", get(fetch_todos))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(Any), // Set specific methods and headers later
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn check_health() -> Html<&'static str> {
    Html("<h1>Works!</h1>")
}

async fn fetch_todos() -> impl IntoResponse {
    //anything that implements IntoResponse can be returned
    let dummy = Json([1, 2, 3, 4, 5]); // no need to call json! since Json seems to automatically serialize it
    return dummy;
}
