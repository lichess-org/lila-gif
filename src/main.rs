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
mod renderer;
mod svg_theme;
mod theme;

use api::{RequestBody, RequestParams};
use renderer::svg_renderer::SVGRenderer;
use theme::Themes;

use crate::renderer::gif_renderer::GIFRenderer;

#[derive(Parser)]
struct Opt {
    /// Listen on this address.
    #[arg(long = "bind", default_value = "127.0.0.1:6175")]
    bind: SocketAddr,
}

async fn image(themes: &'static Themes, Query(req): Query<RequestParams>) -> impl IntoResponse {
    Response::builder()
        .header(CONTENT_TYPE, "image/gif")
        .body(StreamBody::new(stream::iter(
            GIFRenderer::new_image(themes, req).map(Ok::<_, Infallible>),
        )))
        .unwrap()
}

async fn game(themes: &'static Themes, Json(req): Json<RequestBody>) -> impl IntoResponse {
    Response::builder()
        .header(CONTENT_TYPE, "image/gif")
        .body(StreamBody::new(stream::iter(
            GIFRenderer::new_animation(themes, req).map(Ok::<_, Infallible>),
        )))
        .unwrap()
}

async fn example(themes: &'static Themes) -> impl IntoResponse {
    game(themes, Json(RequestBody::example())).await
}

async fn example_svg(Query(req): Query<RequestParams>) -> impl IntoResponse {
    Response::builder()
        .header(CONTENT_TYPE, "image/svg+xml")
        .body(StreamBody::new(stream::iter(
            SVGRenderer::new_image(req).map(Ok::<_, Infallible>),
        )))
        .unwrap()
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let themes: &'static Themes = Box::leak(Box::new(Themes::new()));

    let app = Router::new()
        .route("/image.gif", get(move |req| image(themes, req)))
        .route("/game.gif", post(move |req| game(themes, req)))
        .route("/example.gif", get(move || example(themes)))
        .route("/example.svg", get(move |req| example_svg(req)));

    axum::Server::bind(&opt.bind)
        .serve(app.into_make_service())
        .await
        .expect("bind");
}
