use shakmaty::{Color, Piece, Role};

use crate::assets::{BoardTheme, PieceSet};

pub struct SvgTheme {
    pub board_theme: BoardTheme,
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

const SQUARE_SIZE: usize = 90;

impl SvgTheme {
    pub fn new(board_theme: BoardTheme, piece_set: PieceSet) -> SvgTheme {
        SvgTheme {
            board_theme,
            map: match piece_set {
                PieceSet::Alpha => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/alpha/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/alpha/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/alpha/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/alpha/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/alpha/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/alpha/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/alpha/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/alpha/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/alpha/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/alpha/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/alpha/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/alpha/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Anarcandy => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/anarcandy/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/anarcandy/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/anarcandy/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/anarcandy/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/anarcandy/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/anarcandy/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/anarcandy/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/anarcandy/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/anarcandy/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/anarcandy/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/anarcandy/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/anarcandy/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::California => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/california/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/california/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/california/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/california/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/california/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/california/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/california/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/california/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/california/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/california/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/california/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/california/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Cardinal => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/cardinal/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/cardinal/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/cardinal/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/cardinal/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/cardinal/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/cardinal/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/cardinal/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/cardinal/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/cardinal/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/cardinal/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/cardinal/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/cardinal/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Cburnett => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/cburnett/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/cburnett/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/cburnett/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/cburnett/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/cburnett/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/cburnett/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/cburnett/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/cburnett/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/cburnett/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/cburnett/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/cburnett/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/cburnett/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Chess7 => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/chess7/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/chess7/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/chess7/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/chess7/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/chess7/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/chess7/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/chess7/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/chess7/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/chess7/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/chess7/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/chess7/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/chess7/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Chessnut => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/chessnut/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/chessnut/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/chessnut/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/chessnut/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/chessnut/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/chessnut/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/chessnut/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/chessnut/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/chessnut/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/chessnut/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/chessnut/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/chessnut/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Companion => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/companion/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/companion/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/companion/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/companion/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/companion/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/companion/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/companion/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/companion/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/companion/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/companion/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/companion/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/companion/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Disguised => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/disguised/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/disguised/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/disguised/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/disguised/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/disguised/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/disguised/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/disguised/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/disguised/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/disguised/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/disguised/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/disguised/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/disguised/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Dubrovny => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/dubrovny/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/dubrovny/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/dubrovny/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/dubrovny/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/dubrovny/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/dubrovny/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/dubrovny/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/dubrovny/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/dubrovny/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/dubrovny/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/dubrovny/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/dubrovny/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Fantasy => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/fantasy/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/fantasy/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/fantasy/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/fantasy/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/fantasy/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/fantasy/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/fantasy/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/fantasy/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/fantasy/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/fantasy/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/fantasy/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/fantasy/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Fresca => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/fresca/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/fresca/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/fresca/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/fresca/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/fresca/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/fresca/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/fresca/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/fresca/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/fresca/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/fresca/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/fresca/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/fresca/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Gioco => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/gioco/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/gioco/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/gioco/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/gioco/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/gioco/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/gioco/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/gioco/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/gioco/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/gioco/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/gioco/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/gioco/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/gioco/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Governor => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/governor/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/governor/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/governor/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/governor/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/governor/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/governor/bN.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/governor/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/governor/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/governor/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/governor/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/governor/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/governor/wN.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Horsey => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/horsey/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/horsey/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/horsey/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/horsey/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/horsey/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/horsey/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/horsey/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/horsey/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/horsey/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/horsey/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/horsey/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/horsey/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::IcPieces => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/icpieces/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/icpieces/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/icpieces/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/icpieces/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/icpieces/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/icpieces/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/icpieces/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/icpieces/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/icpieces/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/icpieces/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/icpieces/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/icpieces/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Kosal => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/kosal/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/kosal/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/kosal/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/kosal/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/kosal/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/kosal/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/kosal/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/kosal/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/kosal/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/kosal/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/kosal/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/kosal/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Leipzig => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/leipzig/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/leipzig/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/leipzig/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/leipzig/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/leipzig/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/leipzig/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/leipzig/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/leipzig/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/leipzig/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/leipzig/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/leipzig/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/leipzig/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Letter => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/letter/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/letter/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/letter/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/letter/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/letter/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/letter/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/letter/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/letter/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/letter/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/letter/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/letter/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/letter/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Libra => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/libra/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/libra/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/libra/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/libra/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/libra/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/libra/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/libra/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/libra/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/libra/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/libra/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/libra/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/libra/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Maestro => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/maestro/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/maestro/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/maestro/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/maestro/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/maestro/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/maestro/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/maestro/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/maestro/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/maestro/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/maestro/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/maestro/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/maestro/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Merida => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/merida/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/merida/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/merida/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/merida/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/merida/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/merida/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/merida/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/merida/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/merida/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/merida/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/merida/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/merida/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Pirouetti => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/pirouetti/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/pirouetti/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/pirouetti/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/pirouetti/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/pirouetti/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/pirouetti/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/pirouetti/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/pirouetti/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/pirouetti/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/pirouetti/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/pirouetti/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/pirouetti/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Pixel => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/pixel/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/pixel/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/pixel/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/pixel/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/pixel/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/pixel/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/pixel/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/pixel/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/pixel/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/pixel/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/pixel/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/pixel/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::ReillyCraig => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/reillycraig/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/reillycraig/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/reillycraig/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/reillycraig/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/reillycraig/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/reillycraig/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/reillycraig/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/reillycraig/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/reillycraig/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/reillycraig/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/reillycraig/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/reillycraig/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Riohacha => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/riohacha/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/riohacha/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/riohacha/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/riohacha/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/riohacha/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/riohacha/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/riohacha/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/riohacha/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/riohacha/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/riohacha/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/riohacha/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/riohacha/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Shapes => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/shapes/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/shapes/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/shapes/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/shapes/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/shapes/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/shapes/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/shapes/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/shapes/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/shapes/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/shapes/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/shapes/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/shapes/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Spatial => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/spatial/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/spatial/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/spatial/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/spatial/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/spatial/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/spatial/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/spatial/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/spatial/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/spatial/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/spatial/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/spatial/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/spatial/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Staunty => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/staunty/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/staunty/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/staunty/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/staunty/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/staunty/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/staunty/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/staunty/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/staunty/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/staunty/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/staunty/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/staunty/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/staunty/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Tatiana => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/tatiana/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/tatiana/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/tatiana/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/tatiana/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/tatiana/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/tatiana/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/preprocessed/tatiana/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/preprocessed/tatiana/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/preprocessed/tatiana/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/preprocessed/tatiana/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/preprocessed/tatiana/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/preprocessed/tatiana/wK.svg").to_vec()
                            }
                        },
                    })
                }),
            },
        }
    }
    pub fn square_size(&self) -> usize {
        SQUARE_SIZE
    }
    pub fn chessboard_size(&self) -> usize {
        self.square_size() * 8
    }
    pub fn get_piece(&self, piece: Piece) -> &Vec<u8> {
        self.map
            .by_piece_color(piece.color)
            .by_piece_role(piece.role)
    }
}
