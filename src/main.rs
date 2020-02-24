use std::convert::Infallible;
use std::net::SocketAddr;
use structopt::StructOpt;
use warp::http::status::StatusCode;
use warp::http::Response;
use warp::hyper::Body;
use warp::Filter;

mod api;
mod render;
mod theme;

use api::{RequestBody, RequestParams};
use render::Render;
use theme::Theme;

#[derive(StructOpt)]
struct Opt {
    /// Listen on this address
    #[structopt(long = "address", default_value = "127.0.0.1")]
    address: String,
    /// Listen on this port
    #[structopt(long = "port", default_value = "6175")]
    port: u16
}

fn image(theme: &'static Theme, req: RequestParams) -> impl warp::Reply {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/gif")
        .body(Body::wrap_stream(tokio::stream::iter(Render::new_image(theme, req).map(Ok::<_, Infallible>))))
}

fn animation(theme: &'static Theme, req: RequestBody) -> impl warp::Reply {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/gif")
        .body(Body::wrap_stream(tokio::stream::iter(Render::new_animation(theme, req).map(Ok::<_, Infallible>))))
}

fn example(theme: &'static Theme) -> impl warp::Reply {
    animation(theme, RequestBody::example())
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let bind = SocketAddr::new(opt.address.parse().expect("valid address"), opt.port);

    let theme: &'static Theme = Box::leak(Box::new(Theme::new()));

    let image_route = warp::path!("image.gif")
        .and(warp::get())
        .map(move || theme)
        .and(warp::query::query())
        .map(image);

    let animation_route = warp::path!("game.gif")
        .and(warp::post())
        .map(move || theme)
        .and(warp::body::json())
        .map(animation);

    let example_route = warp::path!("example.gif")
        .and(warp::get())
        .map(move || theme)
        .map(example);

    warp::serve(example_route.or(image_route).or(animation_route))
        .run(bind)
        .await;
}
