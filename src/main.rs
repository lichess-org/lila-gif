use warp::Filter;
use warp::hyper::Body;
use warp::http::Response;
use warp::http::status::StatusCode;
use std::convert::Infallible;

use gift::{Encoder, block};

mod api;
mod theme;
mod render;

use api::{RequestParams, RequestBody, PlayerName, Orientation};
use render::Render;
use theme::{SpriteKey, Theme};

fn image(theme: &'static Theme, req: RequestParams) -> impl warp::Reply {
    /* let params = RequestParams {
        black: Some("revoof".to_owned()),
        white: Some("CM KingsCrusher-YouTube".to_owned()),
        check: None,
        fen: shakmaty::fen::Fen::default(),
        last_move: shakmaty::uci::Uci::Normal { from: Square::E2, to: Square::E4, promotion: None },
        orientation: api::Orientation::White,
    }; */

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/gif")
        .body(Body::wrap_stream(tokio::stream::iter(Render::new_image(theme, req).map(Ok::<_, Infallible>))))
}

fn animation(theme: &'static Theme) -> impl warp::Reply {
    let req = RequestBody {
        black: Some(PlayerName::from("revoof").unwrap()),
        white: Some(PlayerName::from("CM KingsCrusher-YouTube").unwrap()),
        delay: 50,
        frames: vec![],
        orientation: Orientation::White,
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/gif")
        .body(Body::wrap_stream(tokio::stream::iter(Render::new_animation(theme, req).map(Ok::<_, Infallible>))))
}

#[tokio::main]
async fn main() {
    let theme: &'static Theme = Box::leak(Box::new(Theme::new()));

    let routes = warp::get()
        .map(move || theme)
        //.and(warp::query::query())
        .map(animation);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
