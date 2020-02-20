use warp::Filter;
use warp::hyper::Body;
use warp::http::Response;
use warp::http::status::StatusCode;

fn handle() -> impl warp::Reply {
    let stream = tokio::stream::once(Ok::<_, Box<dyn std::error::Error + Send + Sync>>("bar"));
    let stream = tokio::stream::pending::<Result<&'static str, std::convert::Infallible>>();
    let stream = tokio::stream::empty::<Result<&'static str, std::convert::Infallible>>();

    let stream = tokio::stream::iter(std::iter::repeat(
        Ok::<_, std::convert::Infallible>(warp::hyper::body::Bytes::from_static(b"foo\n"))
    ).take(100000).chain(
        std::iter::once(Ok(warp::hyper::body::Bytes::from_static(b"barbar\n")))
    ));

    warp::http::Response::builder()
        .status(warp::http::status::StatusCode::OK)
        .body(Body::wrap_stream(stream))
}

fn image() -> impl warp::Reply {
    let mut output = Vec::new();
    {
        let palette = &[0xff, 0xff, 0xff, 0, 0, 0];

        let bitmap1 = [
            0, 1,
            1, 0
        ];

        let bitmap2 = [
            1, 0,
            0, 1,
        ];

        let mut encoder = gif::Encoder::new(&mut output, 2, 2, palette).expect("encoder");
        use gif::SetParameter;
        encoder.set(gif::Repeat::Infinite).expect("infinite");

        let mut frame = gif::Frame::default();
        frame.width = 2;
        frame.height = 2;
        frame.buffer = std::borrow::Cow::Borrowed(&bitmap1);
        encoder.write_frame(&frame).expect("frame1");

        let mut frame = gif::Frame::default();
        frame.width = 2;
        frame.height = 2;
        frame.delay = 0;
        frame.buffer = std::borrow::Cow::Borrowed(&bitmap2);
        encoder.write_frame(&frame).expect("frame2");
    }

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/gif")
        .body(output)
}

#[tokio::main]
async fn main() {
    let routes = warp::any().map(image);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
