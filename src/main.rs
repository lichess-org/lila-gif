use warp::Filter;
use warp::hyper::Body;
use warp::http::Response;
use warp::http::status::StatusCode;
use std::convert::Infallible;

mod api;
mod theme;
mod render;

use api::{RequestParams, RequestBody};
use render::Render;
use theme::Theme;

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
    let theme: &'static Theme = Box::leak(Box::new(Theme::new()));

    let image_route = warp::get()
        .map(move || theme)
        .and(warp::query::query())
        .map(image);

    let animation_route = warp::post()
        .map(move || theme)
        .and(warp::body::json())
        .map(animation);

    let example_route = warp::get()
        .map(move || theme)
        .map(example);

    warp::serve(example_route.or(image_route).or(animation_route))
        .run(([127, 0, 0, 1], 3030)).await;
}
