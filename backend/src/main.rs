mod database;
mod errors;
mod models;
mod routes;

use axum::{Router, http::HeaderValue, response::Html, routing::get};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use database::init_database;
use routes::todos::todos_router;

use listenfd::ListenFd;

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
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any), // Set specific methods and headers later
        )
        .with_state(conn);

    let mut listenfd = ListenFd::from_env();

    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        } // otherwise fall back to local listening
        None => TcpListener::bind("0.0.0.0:3000").await.unwrap(), // all interfaces, so works on tailscale ip, 
        // no need to be on the same network (well tailscale needs to be running on my phone aswell, so its all interfaces including the tailscale interface i guess)
    };

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}

async fn check_health() -> Html<&'static str> {
    Html("<h1>Works!</h1>")
}
