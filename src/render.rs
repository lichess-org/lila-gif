use bytes::Bytes;
use shakmaty::{Bitboard, Board};

use crate::theme::Theme;
use crate::api::RequestParams;

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
        Render {
            theme,
            buffer: vec![0; theme.width() * theme.height()],
            state: RenderState::Preamble,
            bars: None,
            frames: vec![RenderFrame {
                board: params.fen.board,
                highlighted: Bitboard::EMPTY,
                checked: Bitboard::EMPTY,
            }],
            flipped: false,
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
