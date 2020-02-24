use bytes::{Bytes, BytesMut};
use bytes::buf::ext::BufMutExt as _;
use shakmaty::{Bitboard, Board};
use shakmaty::uci::Uci;
use gift::{Encoder, block};
use std::iter::FusedIterator;
use ndarray::{ArrayViewMut2, s};

use crate::theme::{Theme, SpriteKey};
use crate::api::{PlayerName, RequestParams, Orientation, RequestBody};

enum RenderState {
    Preamble,
    Frame(RenderFrame),
    Complete,
}

struct RenderBars {
    white: PlayerName,
    black: PlayerName,
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
    bars: Option<RenderBars>,
    orientation: Orientation,
    frames: Vec<RenderFrame>,
}

impl Render {
    pub fn new_image(theme: &'static Theme, params: RequestParams) -> Render {
        let bars = params.white.is_some() || params.black.is_some();
        Render {
            theme,
            buffer: vec![0; theme.height(bars) * theme.width()],
            state: RenderState::Preamble,
            bars: if bars {
                Some(RenderBars {
                    white: params.white.unwrap_or_default(),
                    black: params.black.unwrap_or_default(),
                })
            } else {
                None
            },
            frames: vec![RenderFrame {
                board: params.fen.board,
                highlighted: highlight_uci(params.last_move),
                checked: params.check.into_iter().collect(),
                delay: None,
            }],
            orientation: params.orientation,
        }
    }

    pub fn new_animation(theme: &'static Theme, params: RequestBody) -> Render {
        let bars = params.white.is_some() || params.black.is_some();
        Render {
            theme,
            buffer: vec![0; theme.height(bars) * theme.width()],
            state: RenderState::Preamble,
            bars: if bars {
                Some(RenderBars {
                    white: params.white.unwrap_or_default(),
                    black: params.black.unwrap_or_default(),
                })
            } else {
                None
            },
            frames: if params.frames.is_empty() {
                vec![RenderFrame::default()]
            } else {
                let default_delay = params.delay;
                params.frames.into_iter().map(|frame| RenderFrame {
                    board: frame.fen.board,
                    highlighted: highlight_uci(frame.last_move),
                    checked: frame.check.into_iter().collect(),
                    delay: Some(frame.delay.unwrap_or(default_delay)),
                }).collect()
            },
            orientation: params.orientation,
        }
    }
}

impl Iterator for Render {
    type Item = Bytes;

    fn next(&mut self) -> Option<Bytes> {
        let mut output = BytesMut::new().writer();
        match self.state {
            RenderState::Preamble => {
                let mut blocks = Encoder::new(&mut output).into_block_enc();

                blocks.encode(block::Header::default()).expect("enc header");

                blocks.encode(
                    block::LogicalScreenDesc::default()
                        .with_screen_height(self.theme.height(self.bars.is_some()) as u16)
                        .with_screen_width(self.theme.width() as u16)
                        .with_color_table_config(&self.theme.preamble.logical_screen_desc.color_table_config())
                ).expect("enc logical screen desc");

                blocks.encode(
                    self.theme.preamble.global_color_table.clone().expect("color table present")
                ).expect("enc global color table");

                blocks.encode(
                    block::Application::with_loop_count(0)
                ).expect("enc application");

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

                let frame = self.frames.remove(0);

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
                let mut blocks = Encoder::new(&mut output).into_block_enc();

                if self.frames.is_empty() {
                    blocks.encode(block::Trailer::default()).expect("enc trailer");
                    self.state = RenderState::Complete;
                } else {
                    let frame = self.frames.remove(0);

                    let mut ctrl = block::GraphicControl::default();
                    ctrl.set_disposal_method(block::DisposalMethod::Keep);
                    if let Some(delay) = frame.delay {
                        ctrl.set_delay_time_cs(delay);
                    }
                    ctrl.set_transparent_color_idx(self.theme.transparent_color());
                    blocks.encode(ctrl).expect("enc graphic control");

                    let ((x, y), (w, h)) = render_diff(
                        &mut self.buffer,
                        self.theme,
                        self.orientation,
                        Some(&prev),
                        &frame);

                    blocks.encode(
                        block::ImageDesc::default()
                            .with_left(x as u16)
                            .with_top(if self.bars.is_some() { self.theme.bar_height() + y} else { y } as u16)
                            .with_height(h as u16)
                            .with_width(w as u16) // TODO
                    ).expect("enc image desc");

                    let mut image_data = block::ImageData::new(w * h);
                    image_data.add_data(&self.buffer[..(w * h)]);
                    blocks.encode(image_data).expect("enc image data");

                    self.state = RenderState::Frame(frame);
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
            check: frame.checked.contains(sq),
            dark_square: sq.is_dark(),
            last_move: frame.highlighted.contains(sq),
            piece: frame.board.piece_at(sq),
        };

        let real_x = (orientation.x(sq) - x_min) * theme.square();
        let real_y = (orientation.y(sq) - y_min) * theme.square();

        view.slice_mut(
            s!(real_y..(real_y + theme.square()), real_x..(real_x + theme.square()))
        ).assign(&theme.sprite(key));
    }

    ((theme.square() * x_min, theme.square() * y_min), (width, height))
}

fn highlight_uci(uci: Uci) -> Bitboard {
    match uci {
        Uci::Normal { from, to, .. } => Bitboard::from(from) | Bitboard::from(to),
        Uci::Put { to, .. } => Bitboard::from(to),
        Uci::Null => Bitboard::EMPTY,
    }
}
