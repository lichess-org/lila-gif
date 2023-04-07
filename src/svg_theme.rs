use shakmaty::{Color, Piece, Role};

use crate::assets::{ByPieceSet, PieceSet};

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

const SQUARE_SIZE: usize = 90;

impl SvgTheme {
    pub fn new(piece_set: PieceSet) -> SvgTheme {
        SvgTheme {
            map: match piece_set {
                PieceSet::Alpha => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/alpha/bP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/alpha/bN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/alpha/bB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/alpha/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/alpha/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/alpha/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/alpha/wP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/alpha/wN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/alpha/wB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/alpha/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/alpha/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/alpha/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Anarcandy => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/anarcandy/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/anarcandy/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/anarcandy/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/anarcandy/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/anarcandy/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/anarcandy/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/anarcandy/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/anarcandy/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/anarcandy/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/anarcandy/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/anarcandy/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/anarcandy/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::California => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/california/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/california/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/california/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/california/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/california/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/california/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/california/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/california/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/california/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/california/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/california/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/california/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Cardinal => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/cardinal/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/cardinal/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/cardinal/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/cardinal/bR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/cardinal/bQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/cardinal/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/cardinal/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/cardinal/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/cardinal/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/cardinal/wR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/cardinal/wQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/cardinal/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Cburnett => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/cburnett/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/cburnett/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/cburnett/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/cburnett/bR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/cburnett/bQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/cburnett/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/cburnett/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/cburnett/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/cburnett/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/cburnett/wR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/cburnett/wQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/cburnett/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Chess7 => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/chess7/bP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/chess7/bN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/chess7/bB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/chess7/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/chess7/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/chess7/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/chess7/wP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/chess7/wN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/chess7/wB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/chess7/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/chess7/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/chess7/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Chessnut => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/chessnut/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/chessnut/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/chessnut/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/chessnut/bR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/chessnut/bQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/chessnut/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/chessnut/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/chessnut/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/chessnut/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/chessnut/wR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/chessnut/wQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/chessnut/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Companion => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/companion/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/companion/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/companion/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/companion/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/companion/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/companion/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/companion/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/companion/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/companion/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/companion/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/companion/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/companion/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Disguised => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/disguised/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/disguised/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/disguised/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/disguised/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/disguised/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/disguised/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/disguised/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/disguised/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/disguised/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/disguised/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/disguised/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/disguised/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Dubrovny => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/dubrovny/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/dubrovny/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/dubrovny/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/dubrovny/bR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/dubrovny/bQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/dubrovny/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/dubrovny/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/dubrovny/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/dubrovny/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/dubrovny/wR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/dubrovny/wQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/dubrovny/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Fantasy => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/fantasy/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/fantasy/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/fantasy/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/fantasy/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/fantasy/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/fantasy/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/fantasy/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/fantasy/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/fantasy/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/fantasy/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/fantasy/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/fantasy/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Fresca => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/fresca/bP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/fresca/bN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/fresca/bB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/fresca/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/fresca/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/fresca/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/fresca/wP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/fresca/wN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/fresca/wB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/fresca/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/fresca/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/fresca/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Gioco => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/gioco/bP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/gioco/bN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/gioco/bB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/gioco/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/gioco/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/gioco/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/gioco/wP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/gioco/wN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/gioco/wB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/gioco/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/gioco/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/gioco/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Governor => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/governor/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/governor/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/governor/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/governor/bR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/governor/bQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/governor/bN.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/governor/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/governor/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/governor/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/governor/wR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/governor/wQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/governor/wN.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Horsey => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/horsey/bP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/horsey/bN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/horsey/bB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/horsey/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/horsey/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/horsey/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/horsey/wP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/horsey/wN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/horsey/wB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/horsey/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/horsey/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/horsey/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::IcPieces => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/icpieces/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/icpieces/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/icpieces/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/icpieces/bR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/icpieces/bQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/icpieces/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/icpieces/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/icpieces/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/icpieces/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/icpieces/wR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/icpieces/wQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/icpieces/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Kosal => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/kosal/bP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/kosal/bN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/kosal/bB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/kosal/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/kosal/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/kosal/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/kosal/wP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/kosal/wN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/kosal/wB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/kosal/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/kosal/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/kosal/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Leipzig => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/leipzig/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/leipzig/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/leipzig/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/leipzig/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/leipzig/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/leipzig/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/leipzig/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/leipzig/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/leipzig/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/leipzig/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/leipzig/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/leipzig/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Letter => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/letter/bP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/letter/bN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/letter/bB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/letter/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/letter/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/letter/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/letter/wP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/letter/wN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/letter/wB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/letter/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/letter/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/letter/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Libra => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/libra/bP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/libra/bN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/libra/bB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/libra/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/libra/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/libra/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/libra/wP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/libra/wN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/libra/wB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/libra/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/libra/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/libra/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Maestro => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/maestro/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/maestro/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/maestro/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/maestro/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/maestro/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/maestro/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/maestro/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/maestro/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/maestro/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/maestro/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/maestro/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/maestro/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Merida => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/merida/bP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/merida/bN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/merida/bB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/merida/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/merida/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/merida/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/merida/wP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/merida/wN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/merida/wB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/merida/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/merida/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/merida/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Pirouetti => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/pirouetti/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/pirouetti/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/pirouetti/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/pirouetti/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/pirouetti/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/pirouetti/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/pirouetti/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/pirouetti/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/pirouetti/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/pirouetti/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/pirouetti/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/pirouetti/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Pixel => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/pixel/bP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/pixel/bN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/pixel/bB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/pixel/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/pixel/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/pixel/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/pixel/wP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/pixel/wN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/pixel/wB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/pixel/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/pixel/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/pixel/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::ReillyCraig => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/reillycraig/bP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/reillycraig/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/reillycraig/bB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/reillycraig/bR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/reillycraig/bQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/reillycraig/bK.svg").to_vec()
                            }
                        },
                        Color::White => match role {
                            Role::Pawn => {
                                include_bytes!("../theme/piece/reillycraig/wP.svg").to_vec()
                            }
                            Role::Knight => {
                                include_bytes!("../theme/piece/reillycraig/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/reillycraig/wB.svg").to_vec()
                            }
                            Role::Rook => {
                                include_bytes!("../theme/piece/reillycraig/wR.svg").to_vec()
                            }
                            Role::Queen => {
                                include_bytes!("../theme/piece/reillycraig/wQ.svg").to_vec()
                            }
                            Role::King => {
                                include_bytes!("../theme/piece/reillycraig/wK.svg").to_vec()
                            }
                        },
                    })
                }),
                PieceSet::Riohacha => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/riohacha/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/riohacha/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/riohacha/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/riohacha/bR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/riohacha/bQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/riohacha/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/riohacha/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/riohacha/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/riohacha/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/riohacha/wR.svg").to_vec(),
                            Role::Queen => {
                                include_bytes!("../theme/piece/riohacha/wQ.svg").to_vec()
                            }
                            Role::King => include_bytes!("../theme/piece/riohacha/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Shapes => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/shapes/bP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/shapes/bN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/shapes/bB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/shapes/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/shapes/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/shapes/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/shapes/wP.svg").to_vec(),
                            Role::Knight => include_bytes!("../theme/piece/shapes/wN.svg").to_vec(),
                            Role::Bishop => include_bytes!("../theme/piece/shapes/wB.svg").to_vec(),
                            Role::Rook => include_bytes!("../theme/piece/shapes/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/shapes/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/shapes/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Spatial => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/spatial/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/spatial/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/spatial/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/spatial/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/spatial/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/spatial/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/spatial/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/spatial/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/spatial/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/spatial/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/spatial/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/spatial/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Staunty => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/staunty/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/staunty/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/staunty/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/staunty/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/staunty/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/staunty/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/staunty/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/staunty/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/staunty/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/staunty/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/staunty/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/staunty/wK.svg").to_vec(),
                        },
                    })
                }),
                PieceSet::Tatiana => ByPieceColor::new(|color| {
                    ByPieceRole::new(|role| match color {
                        Color::Black => match role {
                            Role::Pawn => include_bytes!("../theme/piece/tatiana/bP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/tatiana/bN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/tatiana/bB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/tatiana/bR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/tatiana/bQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/tatiana/bK.svg").to_vec(),
                        },
                        Color::White => match role {
                            Role::Pawn => include_bytes!("../theme/piece/tatiana/wP.svg").to_vec(),
                            Role::Knight => {
                                include_bytes!("../theme/piece/tatiana/wN.svg").to_vec()
                            }
                            Role::Bishop => {
                                include_bytes!("../theme/piece/tatiana/wB.svg").to_vec()
                            }
                            Role::Rook => include_bytes!("../theme/piece/tatiana/wR.svg").to_vec(),
                            Role::Queen => include_bytes!("../theme/piece/tatiana/wQ.svg").to_vec(),
                            Role::King => include_bytes!("../theme/piece/tatiana/wK.svg").to_vec(),
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

pub struct SvgThemes {
    map: ByPieceSet<SvgTheme>,
}

impl SvgThemes {
    pub fn new() -> SvgThemes {
        SvgThemes {
            map: ByPieceSet::new(|piece_set| SvgTheme::new(piece_set)),
        }
    }
}
