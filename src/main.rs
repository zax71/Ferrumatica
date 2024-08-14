use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/subtract", post(subtract))
        .route("/multiply", post(multiply))
        .route("/divide", post(divide))
        // `POST /users` goes to `create_user`
        .route("/add", post(add));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "An extremely useful API to add, subtract, multiply and divide numbers"
}

async fn add(Json(payload): Json<NumIn>) -> (StatusCode, Json<NumOut>) {
    // insert your application logic here
    let result = NumOut {
        result: payload.x + payload.y,
    };

    return (StatusCode::OK, Json(result));
}

async fn subtract(Json(payload): Json<NumIn>) -> (StatusCode, Json<NumOut>) {
    // insert your application logic here
    let result = NumOut {
        result: payload.x - payload.y,
    };

    return (StatusCode::OK, Json(result));
}

async fn multiply(Json(payload): Json<NumIn>) -> (StatusCode, Json<NumOut>) {
    // insert your application logic here
    let result = NumOut {
        result: payload.x * payload.y,
    };

    return (StatusCode::OK, Json(result));
}

async fn divide(Json(payload): Json<NumIn>) -> (StatusCode, Json<NumOut>) {
    // insert your application logic here
    let result = NumOut {
        result: payload.x / payload.y,
    };

    return (StatusCode::OK, Json(result));
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct NumIn {
    x: f64,
    y: f64,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct NumOut {
    result: f64,
}
