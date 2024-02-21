mod counter;

use axum::{
    response::{Html, IntoResponse}, routing::get, Router
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
    .route("/", get(root))
    .route("/heading", get(heading))
    .route("/note", get(note))
    .route("/beer", get(beer))
    .route("/music", get(music));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn heading() -> impl IntoResponse {
    (
        ([(axum::http::header::CONTENT_TYPE, "image/png")]),
        include_bytes!("obct.png"),
    )
}

async fn note() -> impl IntoResponse {
    (
        ([(axum::http::header::CONTENT_TYPE, "image/svg+xml")]),
        include_bytes!("note.svg"),
    )
}

async fn beer() -> impl IntoResponse {
    (
        ([(axum::http::header::CONTENT_TYPE, "image/svg+xml")]),
        include_bytes!("beer.svg"),
    )
}

async fn music() -> impl IntoResponse {
    (
        ([(axum::http::header::CONTENT_TYPE, "audio/wav")]),
        include_bytes!("music.wav"),
    )
}
