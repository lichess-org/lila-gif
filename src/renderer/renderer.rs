use shakmaty::{uci::Uci, Bitboard, Board, Piece, Role};

pub enum RenderState {
    Preamble,
    Frame(RenderFrame),
    Complete,
}

#[derive(Default, Debug)]
pub struct RenderFrame {
    pub board: Board,
    pub highlighted: Bitboard,
    pub checked: Bitboard,
    pub delay: Option<u16>,
}

impl RenderFrame {
    pub fn diff(&self, prev: &RenderFrame) -> Bitboard {
        (prev.checked ^ self.checked)
            | (prev.highlighted ^ self.highlighted)
            | (prev.board.white() ^ self.board.white())
            | (prev.board.pawns() ^ self.board.pawns())
            | (prev.board.knights() ^ self.board.knights())
            | (prev.board.bishops() ^ self.board.bishops())
            | (prev.board.rooks() ^ self.board.rooks())
            | (prev.board.queens() ^ self.board.queens())
            | (prev.board.kings() ^ self.board.kings())
    }
}

pub fn highlight_uci(uci: Option<Uci>) -> Bitboard {
    match uci {
        Some(Uci::Normal { from, to, .. }) => Bitboard::from(from) | Bitboard::from(to),
        Some(Uci::Put { to, .. }) => Bitboard::from(to),
        _ => Bitboard::EMPTY,
    }
}

pub struct SpriteKey {
    pub piece: Option<Piece>,
    pub dark_square: bool,
    pub highlight: bool,
    pub check: bool,
}

impl SpriteKey {
    pub fn x(&self) -> usize {
        4 * usize::from(self.piece.map_or(false, |p| p.color.is_white()))
            + 2 * usize::from(self.highlight)
            + usize::from(self.dark_square)
    }

    pub fn y(&self) -> usize {
        match self.piece {
            Some(piece) if self.check && piece.role == Role::King => 7,
            Some(piece) => piece.role as usize,
            None => 0,
        }
    }
}
