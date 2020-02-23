use warp::Filter;
use warp::hyper::Body;
use warp::http::Response;
use warp::http::status::StatusCode;

use rusttype::FontCollection;
use rusttype::Scale;
use rusttype::PositionedGlyph;

use shakmaty::{Board, Bitboard, Color};

use ndarray::{ArrayViewMut2, s};

use gift::{Encoder, block};

mod api;
mod theme;

use theme::{SpriteKey, Theme};

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
    let theme = Theme::new();

    let mut output = Vec::new();

    {
        let mut blocks = Encoder::new(&mut output).into_block_enc();
        blocks.encode(block::Header::with_version(*b"89a")).expect("header");

        let color_table_cfg = block::ColorTableConfig::new(
            block::ColorTableExistence::Present,
            block::ColorTableOrdering::NotSorted,
            31
        );

        blocks.encode(
            block::LogicalScreenDesc::default()
                .with_screen_width(theme.width() as u16)
                .with_screen_height(theme.height() as u16)
                .with_color_table_config(&color_table_cfg)
        ).expect("logical screen desc");

        blocks.encode(
            theme.preamble.global_color_table.clone().expect("global color table in theme")
        ).expect("global color table");
    }

    {
        let mut blocks = Encoder::new(&mut output).into_block_enc();
        blocks.encode(
            block::ImageDesc::default()
                .with_width(theme.width() as u16)
                .with_height(theme.height() as u16)
        ).expect("image desc");

        let mut bitmap = vec![0; theme.width() * theme.height()];
        let mut bitmap_view = ArrayViewMut2::from_shape((theme.height(), theme.width()), &mut bitmap).expect("bitmap shape");

        theme.render_bar(
            bitmap_view.slice_mut(s!(..theme.bar_height(), ..)),
            "WIM Kingscrusher-YouTube NaNaNanananannanananan Batman!");

        theme.render_bar(
            bitmap_view.slice_mut(s!((theme.bar_height() + theme.width()).., ..)),
            "revoof");

        let board = Board::new();
        for square in Bitboard::ALL {
            let key = SpriteKey {
                check: false,
                last_move: false,
                dark_square: square.is_dark(),
                piece: board.piece_at(square),
            };

            let flipped = true;
            let size = theme.square();
            let x = size * if flipped { 7 - usize::from(square.file()) } else { usize::from(square.file()) };
            let y = theme.bar_height() + size * if flipped {
                usize::from(square.rank())
            } else {
                7 - usize::from(square.rank())
            };
            bitmap_view.slice_mut(s!(y..(y + size), x..(x + size))).assign(&theme.sprite(key));
        }

        let mut image_data = block::ImageData::new(theme.width() * theme.height());
        image_data.add_data(&bitmap);
        blocks.encode(image_data).expect("image data");
        blocks.encode(block::Trailer::default()).expect("trailer");
    }

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/gif")
        .body(output)
}

#[tokio::main]
async fn main() {
    let _theme = Theme::new();

    let routes = warp::any().map(image);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
