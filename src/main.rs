use std::{convert::Infallible, net::SocketAddr};

use axum::{
    body::Body,
    extract::Query,
    http::header::CONTENT_TYPE,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use futures::stream;
use listenfd::ListenFd;
use tikv_jemallocator::Jemalloc;
use tokio::net::{TcpListener, UnixListener};

mod api;
mod assets;
mod render;
mod theme;

use api::{RequestBody, RequestParams};
use render::Render;
use theme::Themes;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Parser)]
struct Opt {
    /// Listen on this address.
    #[arg(long = "bind", env = "LILA_GIF_BIND", default_value = "127.0.0.1:6175")]
    bind: SocketAddr,
}

async fn image(themes: &'static Themes, Query(req): Query<RequestParams>) -> impl IntoResponse {
    Response::builder()
        .header(CONTENT_TYPE, "image/gif")
        .body(Body::from_stream(stream::iter(
            Render::new_image(themes, req).map(Ok::<_, Infallible>),
        )))
        .unwrap()
}

async fn game(themes: &'static Themes, Json(req): Json<RequestBody>) -> impl IntoResponse {
    Response::builder()
        .header(CONTENT_TYPE, "image/gif")
        .body(Body::from_stream(stream::iter(
            Render::new_animation(themes, req).map(Ok::<_, Infallible>),
        )))
        .unwrap()
}

async fn example(themes: &'static Themes) -> impl IntoResponse {
    game(themes, Json(RequestBody::example())).await
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let themes: &'static Themes = Box::leak(Box::new(Themes::new()));

    let app = Router::new()
        .route("/image.gif", get(move |req| image(themes, req)))
        .route("/game.gif", post(move |req| game(themes, req)))
        .route("/example.gif", get(move || example(themes)));

    let mut fds = ListenFd::from_env();
    if let Ok(Some(uds)) = fds.take_unix_listener(0) {
        uds.set_nonblocking(true).expect("set nonblocking");
        let listener = UnixListener::from_std(uds).expect("listener");
        axum::serve(listener, app).await.expect("serve");
    } else if let Ok(Some(tcp)) = fds.take_tcp_listener(0) {
        tcp.set_nonblocking(true).expect("set nonblocking");
        let listener = TcpListener::from_std(tcp).expect("listener");
        axum::serve(listener, app).await.expect("serve");
    } else {
        let listener = TcpListener::bind(&opt.bind).await.expect("bind");
        axum::serve(listener, app).await.expect("serve");
    }
}
