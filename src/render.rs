use bytes::Bytes;
use shakmaty::Bitboard;

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

struct Render {
    theme: &'static Theme,
    buffer: Vec<u8>,
    state: RenderState,
    bars: Option<RenderBars>,
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
