use bytes::Bytes;
use shakmaty::{Bitboard, Board};
use shakmaty::uci::Uci;

use crate::theme::Theme;
use crate::api::{RequestParams, Orientation};

#[derive(Copy, Clone)]
enum RenderState {
    Preamble,
    Frame,
    Postamble,
    Complete,
}

struct RenderBars {
    white: String,
    black: String,
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

struct Render {
    theme: &'static Theme,
    state: RenderState,
    buffer: Vec<u8>,
    bars: Option<RenderBars>,
    frame: Option<RenderFrame>,
    flipped: bool,
    frames: Vec<RenderFrame>,
}

impl Render {
    pub fn new_image(theme: &'static Theme, params: RequestParams) -> Render {
        let bars = params.white.is_some() || params.black.is_some();
        Render {
            theme,
            buffer: vec![0; theme.width() * if bars { theme.height() } else { theme.width() }],
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
            flipped: params.orientation == Orientation::Black,
            frame: None,
        }
    }
}

impl Iterator for Render {
    type Item = Bytes;

    fn next(&mut self) -> Option<Bytes> {
        match self.state {
            RenderState::Preamble => {
                self.state = RenderState::Postamble;
                Some(Bytes::new())
            }
            _ => None,
        }
    }
}
