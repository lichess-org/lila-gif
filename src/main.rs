use std::{convert::Infallible, net::SocketAddr};

use clap::Parser;
use warp::{
    http::{status::StatusCode, Response},
    hyper::Body,
    Filter,
};

mod api;
mod render;
mod theme;

use api::{RequestBody, RequestParams};
use render::Render;
use theme::Theme;

#[derive(Parser)]
struct Opt {
    /// Listen on this address
    #[clap(long = "address", default_value = "127.0.0.1")]
    address: String,
    /// Listen on this port
    #[clap(long = "port", default_value = "6175")]
    port: u16,
}

fn image(theme: &'static Theme, req: RequestParams) -> impl warp::Reply {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/gif")
        .body(Body::wrap_stream(tokio_stream::iter(
            Render::new_image(theme, req).map(Ok::<_, Infallible>),
        )))
}

fn game(theme: &'static Theme, req: RequestBody) -> impl warp::Reply {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/gif")
        .body(Body::wrap_stream(tokio_stream::iter(
            Render::new_animation(theme, req).map(Ok::<_, Infallible>),
        )))
}

fn example(theme: &'static Theme) -> impl warp::Reply {
    game(theme, RequestBody::example())
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();
    let bind = SocketAddr::new(opt.address.parse().expect("valid address"), opt.port);

    let theme: &'static Theme = Box::leak(Box::new(Theme::new()));

    let image_route = warp::path!("image.gif")
        .and(warp::get())
        .map(move || theme)
        .and(warp::query::query())
        .map(image);

    let game_route = warp::path!("game.gif")
        .and(warp::post())
        .map(move || theme)
        .and(warp::body::json())
        .map(game);

    let example_route = warp::path!("example.gif")
        .and(warp::get())
        .map(move || theme)
        .map(example);

    warp::serve(example_route.or(image_route).or(game_route))
        .run(bind)
        .await;
}
