mod database;
mod errors;
mod models;
mod routes;

use axum::{Router, http::HeaderValue, response::Html, routing::get};
use tower_http::cors::{Any, CorsLayer};

use database::init_database;
use routes::todos::todos_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // dyn std::error::Error is trait object (a way to represent any object that implements a specific trait),
    // dyn is dynamic dispatch when at runtime converts any type of error
    // for example turso::Error into a dyn Error which is then allocated on the heap using Box<> since we dont know the size of
    // the error at compile time

    let conn = init_database().await?;
    let app = Router::new()
        .route("/", get(check_health))
        .nest("/todos", todos_router())
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(Any), // Set specific methods and headers later
        )
        .with_state(conn);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}

async fn check_health() -> Html<&'static str> {
    Html("<h1>Works!</h1>")
}
