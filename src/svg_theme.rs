use shakmaty::{Color, Piece, Role};

use crate::assets::PieceSet;

pub struct SvgTheme {
    map: ByPieceColor<ByPieceRole<Vec<u8>>>,
}

pub struct ByPieceColor<T> {
    inner: [T; 2],
}

impl<T> ByPieceColor<T> {
    pub fn new<F>(f: F) -> ByPieceColor<T>
    where
        F: FnMut(Color) -> T,
    {
        use Color::*;
        ByPieceColor {
            inner: [Black, White].map(f),
        }
    }

    pub fn by_piece_color(&self, color: Color) -> &T {
        &self.inner[color as usize]
    }
}

pub struct ByPieceRole<T> {
    inner: [T; 6],
}

impl<T> ByPieceRole<T> {
    pub fn new<F>(f: F) -> ByPieceRole<T>
    where
        F: FnMut(Role) -> T,
    {
        use Role::*;
        ByPieceRole {
            inner: [Pawn, Knight, Bishop, Rook, Queen, King].map(f),
        }
    }

    pub fn by_piece_role(&self, role: Role) -> &T {
        &self.inner[role as usize - 1]
    }
}

impl SvgTheme {
    pub fn new() -> SvgTheme {
        SvgTheme {
            map: ByPieceColor::new(|color| {
                ByPieceRole::new(|role| match color {
                    Color::Black => match role {
                        Role::Pawn => include_bytes!("../theme/piece/alpha/bP.svg").to_vec(),
                        Role::Knight => include_bytes!("../theme/piece/alpha/bK.svg").to_vec(),
                        Role::Bishop => include_bytes!("../theme/piece/alpha/bB.svg").to_vec(),
                        Role::Rook => include_bytes!("../theme/piece/alpha/bR.svg").to_vec(),
                        Role::Queen => include_bytes!("../theme/piece/alpha/bQ.svg").to_vec(),
                        Role::King => include_bytes!("../theme/piece/alpha/bK.svg").to_vec(),
                    },
                    Color::White => match role {
                        Role::Pawn => include_bytes!("../theme/piece/alpha/wP.svg").to_vec(),
                        Role::Knight => include_bytes!("../theme/piece/alpha/wK.svg").to_vec(),
                        Role::Bishop => include_bytes!("../theme/piece/alpha/wB.svg").to_vec(),
                        Role::Rook => include_bytes!("../theme/piece/alpha/wR.svg").to_vec(),
                        Role::Queen => include_bytes!("../theme/piece/alpha/wQ.svg").to_vec(),
                        Role::King => include_bytes!("../theme/piece/alpha/wK.svg").to_vec(),
                    },
                })
            }),
        }
    }
    pub fn get_piece(&self, piece: Piece) -> &Vec<u8> {
        self.map
            .by_piece_color(piece.color)
            .by_piece_role(piece.role)
    }
}
