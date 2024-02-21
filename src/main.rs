mod counter;

use std::{sync::Arc, time::Duration};

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use counter::Counter;
use tokio::{task, time};

#[tokio::main]
async fn main() {
    let counter = Arc::new(Counter::new());

    let cloned_counter = counter.clone();
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;
            cloned_counter.update();
        }
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/heading", get(heading))
        .route("/note", get(note))
        .route("/beer", get(beer))
        .route("/music", get(music))
        .route("/value", get(value))
        .with_state(counter.clone())
        .route("/reset", get(reset))
        .with_state(counter);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    forever.await.unwrap();
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

async fn value(State(state): State<Arc<Counter>>) -> String {
    state.get().to_string()
}

async fn reset(State(state): State<Arc<Counter>>) {
    state.decrease(state.get());
}
