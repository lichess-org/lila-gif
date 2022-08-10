use std::{convert::Infallible, net::SocketAddr};

use axum::{
    body::StreamBody,
    extract::Query,
    http::header::CONTENT_TYPE,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use futures::stream;

mod api;
mod assets;
mod render;
mod theme;

use api::{RequestBody, RequestParams};
use render::Render;
use theme::ThemeMap;

#[derive(Parser)]
struct Opt {
    /// Listen on this address.
    #[clap(long = "bind", default_value = "127.0.0.1:6175")]
    bind: SocketAddr,
}

async fn image(
    theme_map: &'static ThemeMap,
    Query(req): Query<RequestParams>,
) -> impl IntoResponse {
    Response::builder()
        .header(CONTENT_TYPE, "image/gif")
        .body(StreamBody::new(stream::iter(
            Render::new_image(theme_map, req).map(Ok::<_, Infallible>),
        )))
        .unwrap()
}

async fn game(theme_map: &'static ThemeMap, Json(req): Json<RequestBody>) -> impl IntoResponse {
    Response::builder()
        .header(CONTENT_TYPE, "image/gif")
        .body(StreamBody::new(stream::iter(
            Render::new_animation(theme_map, req).map(Ok::<_, Infallible>),
        )))
        .unwrap()
}

async fn example(theme_map: &'static ThemeMap) -> impl IntoResponse {
    game(theme_map, Json(RequestBody::example())).await
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let theme_map: &'static ThemeMap = Box::leak(Box::new(ThemeMap::new().initialize()));

    let app = Router::new()
        .route("/image.gif", get(move |req| image(theme_map, req)))
        .route("/game.gif", post(move |req| game(theme_map, req)))
        .route("/example.gif", get(move || example(theme_map)));

    axum::Server::bind(&opt.bind)
        .serve(app.into_make_service())
        .await
        .expect("bind");
}
