use warp::Filter;
use warp::hyper::Body;
use warp::http::Response;
use warp::http::status::StatusCode;

use rusttype::FontCollection;
use rusttype::Scale;
use rusttype::PositionedGlyph;

mod api;
mod theme;

use theme::Theme;

const SIZE: usize = 90;
const LINE_HEIGHT: usize = 50;

/* struct Theme {
}

struct SpriteKey {
    piece: piece,
    light: bool,
    check: bool,
}

impl Theme {
    fn load() -> Theme {
    }

    fn sprite(self, key: SpriteKey) -> Spite {
    }

    fn transparent(self) -> u8 {
        2
    }

    fn darkest(self) -> u8 {
        0
    }

    fn lightest(self) -> u8 {
        1
    }
}

struct RequestBody {
    white: String,
    black: String,
    frames: Vec<RequestFrame>,
    outcome: Option<Outcome>,
}

struct RequestFrame {
    fen: Fen,
    duration: Option<u16>,
    m: Option<Uci>,
    check: Option<Square>,
}

#[derive(Copy, Clone)]
enum BlockIteratorState {
    Preamble,
    Frames,
    Postamble,
}

struct BlockIterator {
    board: Board,
    state: GameResponseState,
}

impl BlockIterator {
    fn new(req: GameRequest) -> GameResponse {
        GameResponse {
            board: !,
            state: GameResponseState::Preamble,
        }
    }
}

impl Iterator for BlockIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.state = match self.state {
            GameResponseState::Preamble => GameResponseState::Frames,
            GameResponseState::Frames => GameResponseState::Postamble,
            GameResponseState::Postamble => return None,
        };
    }
} */

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

        let mut bitmap1 = vec![0; SIZE * 8 * (SIZE * 8 + LINE_HEIGHT * 2)];

        let bitmap2 = [
            1, 0,
            0, 1,
        ];

        let mut encoder = gif::Encoder::new(&mut output, 2, 2, palette).expect("encoder");
        use gif::SetParameter;
        encoder.set(gif::Repeat::Infinite).expect("infinite");

        // https://gitlab.redox-os.org/redox-os/rusttype/blob/master/dev/examples/simple.rs
        let font_data = include_bytes!("../theme/NotoSans-Regular.ttf");
        let collection = FontCollection::from_bytes(font_data as &[u8]).expect("font collection");
        let font = collection.into_font().expect("single font");
        let height = 12.4f32 * 10.0;
        let pixel_height = height.ceil() as usize;
        let scale = Scale {
            x: LINE_HEIGHT as f32,
            y: LINE_HEIGHT as f32,
        };
        let v_metrics = font.v_metrics(scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);
        let glyphs: Vec<PositionedGlyph<'_>> = font.layout("Rust simpl", scale, offset).collect();

        let mut base_x = 0;
        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    if v > 0.2 {
                        bitmap1[(y + bb.min.y as u32) as usize * SIZE * 8 + bb.min.x as usize + x as usize] = 1;
                    }
                });
                base_x += bb.max.x;
            }
        }

        let mut frame = gif::Frame::default();
        frame.width = (SIZE * 8) as u16;
        frame.height = (SIZE * 8 + LINE_HEIGHT * 2) as u16;
        frame.buffer = std::borrow::Cow::Borrowed(&bitmap1);
        encoder.write_frame(&frame).expect("frame1");

        /* let mut frame = gif::Frame::default();
        frame.width = 2;
        frame.height = 2;
        frame.delay = 0;
        frame.buffer = std::borrow::Cow::Borrowed(&bitmap2);
        encoder.write_frame(&frame).expect("frame2"); */
    }

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/gif")
        .body(output)
}

#[tokio::main]
async fn main() {
    let theme = Theme::new();

    let routes = warp::any().map(image);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
