use bytes::buf::ext::BufMutExt as _;
use bytes::{Bytes, BytesMut};
use gift::{block, Encoder};
use ndarray::{s, ArrayViewMut2};
use shakmaty::uci::Uci;
use shakmaty::{Bitboard, Board};
use std::vec;
use std::iter::FusedIterator;

use crate::api::{Orientation, PlayerName, Comment, RequestBody, RequestParams};
use crate::theme::{SpriteKey, Theme};

enum RenderState {
    Preamble,
    Frame(RenderFrame),
    Complete,
}

struct PlayerBars {
    white: PlayerName,
    black: PlayerName,
}

impl PlayerBars {
    fn from(white: Option<PlayerName>, black: Option<PlayerName>) -> Option<PlayerBars> {
        if white.is_some() || black.is_some() {
            Some(PlayerBars {
                white: white.unwrap_or_default(),
                black: black.unwrap_or_default(),
            })
        } else {
            None
        }
    }
}

#[derive(Default)]
struct RenderFrame {
    board: Board,
    highlighted: Bitboard,
    checked: Bitboard,
    delay: Option<u16>,
}

impl RenderFrame {
    fn diff(&self, prev: &RenderFrame) -> Bitboard {
        (prev.checked ^ self.checked) |
        (prev.highlighted ^ self.highlighted) |
        (prev.board.white() ^ self.board.white()) |
        (prev.board.pawns() ^ self.board.pawns()) |
        (prev.board.knights() ^ self.board.knights()) |
        (prev.board.bishops() ^ self.board.bishops()) |
        (prev.board.rooks() ^ self.board.rooks()) |
        (prev.board.queens() ^ self.board.queens()) |
        (prev.board.kings() ^ self.board.kings())
    }
}

pub struct Render {
    theme: &'static Theme,
    state: RenderState,
    buffer: Vec<u8>,
    comment: Option<Comment>,
    bars: Option<PlayerBars>,
    orientation: Orientation,
    frames: vec::IntoIter<RenderFrame>,
}

impl Render {
    pub fn new_image(theme: &'static Theme, params: RequestParams) -> Render {
        let bars = params.white.is_some() || params.black.is_some();
        Render {
            theme,
            buffer: vec![0; theme.height(bars) * theme.width()],
            state: RenderState::Preamble,
            comment: params.comment,
            bars: PlayerBars::from(params.white, params.black),
            orientation: params.orientation,
            frames: vec![RenderFrame {
                board: params.fen.board,
                highlighted: highlight_uci(params.last_move),
                checked: params.check.into_iter().collect(),
                delay: None,
            }].into_iter(),
        }
    }

    pub fn new_animation(theme: &'static Theme, params: RequestBody) -> Render {
        let bars = params.white.is_some() || params.black.is_some();
        let default_delay = params.delay;
        Render {
            theme,
            buffer: vec![0; theme.height(bars) * theme.width()],
            state: RenderState::Preamble,
            comment: params.comment,
            bars: PlayerBars::from(params.white, params.black),
            orientation: params.orientation,
            frames: params.frames.into_iter().map(|frame| RenderFrame {
                board: frame.fen.board,
                highlighted: highlight_uci(frame.last_move),
                checked: frame.check.into_iter().collect(),
                delay: Some(frame.delay.unwrap_or(default_delay)),
            }).collect::<Vec<_>>().into_iter()
        }
    }
}

impl Iterator for Render {
    type Item = Bytes;

    fn next(&mut self) -> Option<Bytes> {
        let mut output = BytesMut::new().writer();
        match self.state {
            RenderState::Preamble => {
                let mut blocks = Encoder::new_unbuffered(&mut output).into_block_enc();

                blocks.encode(block::Header::default()).expect("enc header");

                blocks.encode(
                    block::LogicalScreenDesc::default()
                        .with_screen_height(self.theme.height(self.bars.is_some()) as u16)
                        .with_screen_width(self.theme.width() as u16)
                        .with_color_table_config(&self.theme.color_table_config())
                ).expect("enc logical screen desc");

                blocks.encode(
                    self.theme.global_color_table().clone()
                ).expect("enc global color table");

                blocks.encode(
                    block::Application::with_loop_count(0)
                ).expect("enc application");

                let mut comments = block::Comment::default();
                comments.add_comment(self.comment.as_ref().map_or(
                    "https://github.com/niklasf/lila-gif".as_bytes(),
                    |c| c.as_bytes()
                ));
                blocks.encode(comments).expect("enc comment");

                let mut view = ArrayViewMut2::from_shape(
                    (self.theme.height(self.bars.is_some()), self.theme.width()),
                    &mut self.buffer
                ).expect("shape");

                let mut board_view = if let Some(ref bars) = self.bars {
                    self.theme.render_bar(
                        view.slice_mut(s!(..self.theme.bar_height(), ..)),
                        self.orientation.fold(&bars.black, &bars.white));

                    self.theme.render_bar(
                        view.slice_mut(s!((self.theme.bar_height() + self.theme.width()).., ..)),
                        self.orientation.fold(&bars.white, &bars.black));

                    view.slice_mut(s!(self.theme.bar_height()..(self.theme.bar_height() + self.theme.width()), ..))
                } else {
                    view
                };

                let frame = self.frames.next().unwrap_or_default();

                if let Some(delay) = frame.delay {
                    let mut ctrl = block::GraphicControl::default();
                    ctrl.set_delay_time_cs(delay);
                    blocks.encode(ctrl).expect("enc graphic control");
                }

                render_diff(
                    board_view.as_slice_mut().expect("continguous"),
                    self.theme,
                    self.orientation,
                    None,
                    &frame);

                blocks.encode(
                    block::ImageDesc::default()
                        .with_height(self.theme.height(self.bars.is_some()) as u16)
                        .with_width(self.theme.width() as u16)
                ).expect("enc image desc");

                let mut image_data = block::ImageData::new(self.buffer.len());
                image_data.add_data(&self.buffer);
                blocks.encode(image_data).expect("enc image data");

                self.state = RenderState::Frame(frame);
            }
            RenderState::Frame(ref prev) => {
                let mut blocks = Encoder::new_unbuffered(&mut output).into_block_enc();

                if let Some(frame) = self.frames.next() {
                    let mut ctrl = block::GraphicControl::default();
                    ctrl.set_disposal_method(block::DisposalMethod::Keep);
                    ctrl.set_transparent_color_idx(self.theme.transparent_color());
                    if let Some(delay) = frame.delay {
                        ctrl.set_delay_time_cs(delay);
                    }
                    blocks.encode(ctrl).expect("enc graphic control");

                    let ((left, y), (w, h)) = render_diff(
                        &mut self.buffer,
                        self.theme,
                        self.orientation,
                        Some(&prev),
                        &frame);

                    let top = y + if self.bars.is_some() { self.theme.bar_height() } else { 0 };

                    blocks.encode(
                        block::ImageDesc::default()
                            .with_left(left as u16)
                            .with_top(top as u16)
                            .with_height(h as u16)
                            .with_width(w as u16)
                    ).expect("enc image desc");

                    let mut image_data = block::ImageData::new(w * h);
                    image_data.add_data(&self.buffer[..(w * h)]);
                    blocks.encode(image_data).expect("enc image data");

                    self.state = RenderState::Frame(frame);
                } else {
                    blocks.encode(block::Trailer::default()).expect("enc trailer");
                    self.state = RenderState::Complete;
                }
            }
            RenderState::Complete => return None,
        }
        Some(output.into_inner().freeze())
    }
}

impl FusedIterator for Render { }

fn render_diff(buffer: &mut [u8], theme: &Theme, orientation: Orientation, prev: Option<&RenderFrame>, frame: &RenderFrame) -> ((usize, usize), (usize, usize)) {
    let diff = if let Some(prev) = prev {
        prev.diff(frame)
    } else {
        Bitboard::ALL
    };

    if diff.is_empty() {
        return ((0, 0), (0, 0));
    }

    let x_min = diff.into_iter().map(|sq| orientation.x(sq)).min().unwrap();
    let x_max = diff.into_iter().map(|sq| orientation.x(sq)).max().unwrap();
    let y_min = diff.into_iter().map(|sq| orientation.y(sq)).min().unwrap();
    let y_max = diff.into_iter().map(|sq| orientation.y(sq)).max().unwrap();

    let width = theme.square() * (x_max - x_min + 1);
    let height = theme.square() * (y_max - y_min + 1);

    let mut view = ArrayViewMut2::from_shape((height, width), buffer).expect("shape");

    if prev.is_some() {
        view.fill(theme.transparent_color());
    }

    for sq in diff {
        let key = SpriteKey {
            piece: frame.board.piece_at(sq),
            dark_square: sq.is_dark(),
            highlight: frame.highlighted.contains(sq),
            check: frame.checked.contains(sq),
        };

        let view_x = (orientation.x(sq) - x_min) * theme.square();
        let view_y = (orientation.y(sq) - y_min) * theme.square();

        view.slice_mut(
            s!(view_y..(view_y + theme.square()), view_x..(view_x + theme.square()))
        ).assign(&theme.sprite(key));
    }

    ((theme.square() * x_min, theme.square() * y_min), (width, height))
}

fn highlight_uci(uci: Option<Uci>) -> Bitboard {
    match uci {
        Some(Uci::Normal { from, to, .. }) => Bitboard::from(from) | Bitboard::from(to),
        Some(Uci::Put { to, .. }) => Bitboard::from(to),
        _ => Bitboard::EMPTY,
    }
}
