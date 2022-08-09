use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::Query,
    http::header::CONTENT_TYPE,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use clap::Parser;

mod api;
mod render;
mod theme;

use api::{RequestBody, RequestParams};
use render::Render;
use theme::Theme;

#[derive(Parser)]
struct Opt {
    /// Listen on this address.
    #[clap(long = "bind", default_value = "127.0.0.1:6175")]
    bind: SocketAddr,
}

async fn image(theme: &'static Theme, Query(req): Query<RequestParams>) -> impl IntoResponse {
    Response::builder()
        .header(CONTENT_TYPE, "image/gif")
        .body(Body::from(Render::new_image(theme, req).render()))
        .unwrap()
}

async fn game(theme: &'static Theme, Json(req): Json<RequestBody>) -> impl IntoResponse {
    Response::builder()
        .header(CONTENT_TYPE, "image/gif")
        .body(Body::from(Render::new_animation(theme, req).render()))
        .unwrap()
}

async fn example(theme: &'static Theme) -> impl IntoResponse {
    game(theme, Json(RequestBody::example())).await
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let theme: &'static Theme = Box::leak(Box::new(Theme::new()));

    let app = Router::new()
        .route("/image.gif", get(move |req| image(theme, req)))
        .route("/game.gif", post(move |req| game(theme, req)))
        .route("/example.gif", get(move || example(theme)));

    axum::Server::bind(&opt.bind)
        .serve(app.into_make_service())
        .await
        .expect("bind");
}
