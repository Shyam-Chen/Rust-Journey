use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct HelloWorld {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct HelloWorldPayload {
    name: String,
}

async fn get_hello_world() -> Json<HelloWorld> {
    Json(HelloWorld {
        message: "Hello, World!".into(),
    })
}

async fn post_hello_world(Json(payload): Json<HelloWorldPayload>) -> Json<HelloWorld> {
    let message = format!("Hello, {}!", payload.name);
    Json(HelloWorld { message })
}

pub fn router() -> Router {
    Router::new().nest(
        "/hello-world",
        Router::new()
            .route("/", get(get_hello_world))
            .route("/", post(post_hello_world)),
    )
}
