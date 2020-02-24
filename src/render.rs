use bytes::{Bytes, BytesMut};
use bytes::buf::ext::BufMutExt as _;
use shakmaty::{Bitboard, Board};
use shakmaty::uci::Uci;
use gift::{Encoder, block};
use std::convert::Infallible;
use std::iter::FusedIterator;
use ndarray::{ArrayViewMut2, s};

use crate::theme::Theme;
use crate::api::{PlayerName, RequestParams, Orientation};

#[derive(Copy, Clone)]
enum RenderState {
    Preamble,
    Frame,
    Complete,
}

struct RenderBars {
    white: PlayerName,
    black: PlayerName,
}

struct RenderFrame {
    board: Board,
    highlighted: Bitboard,
    checked: Bitboard,
}

impl RenderFrame {
    fn diff(&self, prev: RenderFrame) -> Bitboard {
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
    frame: Option<RenderFrame>,
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
                highlighted: match params.last_move {
                    Uci::Normal { from, to, .. } => Bitboard::from(from) | Bitboard::from(to),
                    Uci::Null => Bitboard::EMPTY,
                    Uci::Put { to, .. } => Bitboard::from(to),
                },
                checked: params.check.into_iter().collect(),
            }],
            orientation: params.orientation,
            frame: None,
        }
    }
}

impl Iterator for Render {
    type Item = Bytes;

    fn next(&mut self) -> Option<Bytes> {
        let mut output = BytesMut::new().writer();
        match self.state {
            RenderState::Preamble => {
                self.state = RenderState::Complete; // XXX
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

                let mut view = ArrayViewMut2::from_shape(
                    (self.theme.height(self.bars.is_some()), self.theme.width()),
                    &mut self.buffer
                ).expect("shape");

                if let Some(ref bars) = self.bars {
                    self.theme.render_bar(
                        view.slice_mut(s!(..self.theme.bar_height(), ..)),
                        self.orientation.fold(&bars.black, &bars.white));

                    self.theme.render_bar(
                        view.slice_mut(s!((self.theme.bar_height() + self.theme.width()).., ..)),
                        self.orientation.fold(&bars.white, &bars.black));
                }

                blocks.encode(
                    block::ImageDesc::default()
                        .with_height(self.theme.height(self.bars.is_some()) as u16)
                        .with_width(self.theme.width() as u16)
                ).expect("enc image desc");

                let mut image_data = block::ImageData::new(self.buffer.len());
                image_data.add_data(&self.buffer);
                blocks.encode(image_data).expect("enc image data");
            }
            RenderState::Frame => {
                let mut blocks = Encoder::new(&mut output).into_block_enc();
                if self.frames.is_empty() {
                    self.state = RenderState::Complete;
                    blocks.encode(block::Trailer::default()).expect("enc trailer");
                } else {
                    let frame = self.frames.remove(0);
                    let mut image_data = block::ImageData::new(self.buffer.len());
                    image_data.add_data(&self.buffer);

                    blocks.encode(
                        block::ImageDesc::default()
                            .with_height(self.theme.height(self.bars.is_some()) as u16)
                            .with_width(self.theme.width() as u16)
                    ).expect("enc image desc");

                    blocks.encode(image_data).expect("enc image data");
                }
            }
            RenderState::Complete => return None,
        }
        Some(output.into_inner().freeze())
    }
}

impl FusedIterator for Render { }
