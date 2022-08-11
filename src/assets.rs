use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BoardTheme {
    Blue,
    Brown,
    Green,
    Ic,
    Purple,
}

pub struct ByBoardTheme<T> {
    inner: [T; 5],
}

impl<T> ByBoardTheme<T> {
    pub fn new<F>(f: F) -> ByBoardTheme<T>
    where
        F: FnMut(BoardTheme) -> T,
    {
        use BoardTheme::*;
        ByBoardTheme {
            inner: [Blue, Brown, Green, Ic, Purple].map(f),
        }
    }

    pub fn by_board_theme(&self, board: BoardTheme) -> &T {
        &self.inner[board as usize]
    }
}

impl Default for BoardTheme {
    fn default() -> BoardTheme {
        BoardTheme::Brown
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PieceSet {
    Alpha,
    Anarcandy,
    California,
    Cardinal,
    Cburnett,
    Chess7,
    Chessnut,
    Companion,
    Dubrovny,
    Fantasy,
    Fresca,
    Gioco,
    Governor,
    Horsey,
    IcPieces,
    Kosal,
    Leipzig,
    Letter,
    Libra,
    Maestro,
    Merida,
    Pirouetti,
    Pixel,
    ReillyCraig,
    Riohacha,
    Shapes,
    Spatial,
    Staunty,
    Tatiana,
}

impl Default for PieceSet {
    fn default() -> PieceSet {
        PieceSet::Cburnett
    }
}

pub struct ByPieceSet<T> {
    inner: [T; 29],
}

impl<T> ByPieceSet<T> {
    pub fn new<F>(f: F) -> ByPieceSet<T>
    where
        F: FnMut(PieceSet) -> T,
    {
        use PieceSet::*;
        ByPieceSet {
            inner: [
                Alpha,
                Anarcandy,
                California,
                Cardinal,
                Cburnett,
                Chess7,
                Chessnut,
                Companion,
                Dubrovny,
                Fantasy,
                Fresca,
                Gioco,
                Governor,
                Horsey,
                IcPieces,
                Kosal,
                Leipzig,
                Letter,
                Libra,
                Maestro,
                Merida,
                Pirouetti,
                Pixel,
                ReillyCraig,
                Riohacha,
                Shapes,
                Spatial,
                Staunty,
                Tatiana,
            ]
            .map(f),
        }
    }

    pub fn by_piece_set(&self, piece_set: PieceSet) -> &T {
        &self.inner[piece_set as usize]
    }
}

pub fn sprite_data(board: BoardTheme, pieces: PieceSet) -> &'static [u8] {
    use PieceSet::*;
    match board {
        BoardTheme::Blue => match pieces {
            Alpha => include_bytes!("../theme/sprites/blue-alpha.gif"),
            Anarcandy => include_bytes!("../theme/sprites/blue-anarcandy.gif"),
            California => include_bytes!("../theme/sprites/blue-california.gif"),
            Cardinal => include_bytes!("../theme/sprites/blue-cardinal.gif"),
            Cburnett => include_bytes!("../theme/sprites/blue-cburnett.gif"),
            Chess7 => include_bytes!("../theme/sprites/blue-chess7.gif"),
            Chessnut => include_bytes!("../theme/sprites/blue-chessnut.gif"),
            Companion => include_bytes!("../theme/sprites/blue-companion.gif"),
            Dubrovny => include_bytes!("../theme/sprites/blue-dubrovny.gif"),
            Fantasy => include_bytes!("../theme/sprites/blue-fantasy.gif"),
            Fresca => include_bytes!("../theme/sprites/blue-fresca.gif"),
            Gioco => include_bytes!("../theme/sprites/blue-gioco.gif"),
            Governor => include_bytes!("../theme/sprites/blue-governor.gif"),
            Horsey => include_bytes!("../theme/sprites/blue-horsey.gif"),
            IcPieces => include_bytes!("../theme/sprites/blue-icpieces.gif"),
            Kosal => include_bytes!("../theme/sprites/blue-kosal.gif"),
            Leipzig => include_bytes!("../theme/sprites/blue-leipzig.gif"),
            Letter => include_bytes!("../theme/sprites/blue-letter.gif"),
            Libra => include_bytes!("../theme/sprites/blue-libra.gif"),
            Maestro => include_bytes!("../theme/sprites/blue-maestro.gif"),
            Merida => include_bytes!("../theme/sprites/blue-merida.gif"),
            Pirouetti => include_bytes!("../theme/sprites/blue-pirouetti.gif"),
            Pixel => include_bytes!("../theme/sprites/blue-pixel.gif"),
            ReillyCraig => include_bytes!("../theme/sprites/blue-reillycraig.gif"),
            Riohacha => include_bytes!("../theme/sprites/blue-riohacha.gif"),
            Shapes => include_bytes!("../theme/sprites/blue-shapes.gif"),
            Spatial => include_bytes!("../theme/sprites/blue-spatial.gif"),
            Staunty => include_bytes!("../theme/sprites/blue-staunty.gif"),
            Tatiana => include_bytes!("../theme/sprites/blue-tatiana.gif"),
        },
        BoardTheme::Brown => match pieces {
            Alpha => include_bytes!("../theme/sprites/brown-alpha.gif"),
            Anarcandy => include_bytes!("../theme/sprites/brown-anarcandy.gif"),
            California => include_bytes!("../theme/sprites/brown-california.gif"),
            Cardinal => include_bytes!("../theme/sprites/brown-cardinal.gif"),
            Cburnett => include_bytes!("../theme/sprites/brown-cburnett.gif"),
            Chess7 => include_bytes!("../theme/sprites/brown-chess7.gif"),
            Chessnut => include_bytes!("../theme/sprites/brown-chessnut.gif"),
            Companion => include_bytes!("../theme/sprites/brown-companion.gif"),
            Dubrovny => include_bytes!("../theme/sprites/brown-dubrovny.gif"),
            Fantasy => include_bytes!("../theme/sprites/brown-fantasy.gif"),
            Fresca => include_bytes!("../theme/sprites/brown-fresca.gif"),
            Gioco => include_bytes!("../theme/sprites/brown-gioco.gif"),
            Governor => include_bytes!("../theme/sprites/brown-governor.gif"),
            Horsey => include_bytes!("../theme/sprites/brown-horsey.gif"),
            IcPieces => include_bytes!("../theme/sprites/brown-icpieces.gif"),
            Kosal => include_bytes!("../theme/sprites/brown-kosal.gif"),
            Leipzig => include_bytes!("../theme/sprites/brown-leipzig.gif"),
            Letter => include_bytes!("../theme/sprites/brown-letter.gif"),
            Libra => include_bytes!("../theme/sprites/brown-libra.gif"),
            Maestro => include_bytes!("../theme/sprites/brown-maestro.gif"),
            Merida => include_bytes!("../theme/sprites/brown-merida.gif"),
            Pirouetti => include_bytes!("../theme/sprites/brown-pirouetti.gif"),
            Pixel => include_bytes!("../theme/sprites/brown-pixel.gif"),
            ReillyCraig => include_bytes!("../theme/sprites/brown-reillycraig.gif"),
            Riohacha => include_bytes!("../theme/sprites/brown-riohacha.gif"),
            Shapes => include_bytes!("../theme/sprites/brown-shapes.gif"),
            Spatial => include_bytes!("../theme/sprites/brown-spatial.gif"),
            Staunty => include_bytes!("../theme/sprites/brown-staunty.gif"),
            Tatiana => include_bytes!("../theme/sprites/brown-tatiana.gif"),
        },
        BoardTheme::Green => match pieces {
            Alpha => include_bytes!("../theme/sprites/green-alpha.gif"),
            Anarcandy => include_bytes!("../theme/sprites/green-anarcandy.gif"),
            California => include_bytes!("../theme/sprites/green-california.gif"),
            Cardinal => include_bytes!("../theme/sprites/green-cardinal.gif"),
            Cburnett => include_bytes!("../theme/sprites/green-cburnett.gif"),
            Chess7 => include_bytes!("../theme/sprites/green-chess7.gif"),
            Chessnut => include_bytes!("../theme/sprites/green-chessnut.gif"),
            Companion => include_bytes!("../theme/sprites/green-companion.gif"),
            Dubrovny => include_bytes!("../theme/sprites/green-dubrovny.gif"),
            Fantasy => include_bytes!("../theme/sprites/green-fantasy.gif"),
            Fresca => include_bytes!("../theme/sprites/green-fresca.gif"),
            Gioco => include_bytes!("../theme/sprites/green-gioco.gif"),
            Governor => include_bytes!("../theme/sprites/green-governor.gif"),
            Horsey => include_bytes!("../theme/sprites/green-horsey.gif"),
            IcPieces => include_bytes!("../theme/sprites/green-icpieces.gif"),
            Kosal => include_bytes!("../theme/sprites/green-kosal.gif"),
            Leipzig => include_bytes!("../theme/sprites/green-leipzig.gif"),
            Letter => include_bytes!("../theme/sprites/green-letter.gif"),
            Libra => include_bytes!("../theme/sprites/green-libra.gif"),
            Maestro => include_bytes!("../theme/sprites/green-maestro.gif"),
            Merida => include_bytes!("../theme/sprites/green-merida.gif"),
            Pirouetti => include_bytes!("../theme/sprites/green-pirouetti.gif"),
            Pixel => include_bytes!("../theme/sprites/green-pixel.gif"),
            ReillyCraig => include_bytes!("../theme/sprites/green-reillycraig.gif"),
            Riohacha => include_bytes!("../theme/sprites/green-riohacha.gif"),
            Shapes => include_bytes!("../theme/sprites/green-shapes.gif"),
            Spatial => include_bytes!("../theme/sprites/green-spatial.gif"),
            Staunty => include_bytes!("../theme/sprites/green-staunty.gif"),
            Tatiana => include_bytes!("../theme/sprites/green-tatiana.gif"),
        },
        BoardTheme::Ic => match pieces {
            Alpha => include_bytes!("../theme/sprites/ic-alpha.gif"),
            Anarcandy => include_bytes!("../theme/sprites/ic-anarcandy.gif"),
            California => include_bytes!("../theme/sprites/ic-california.gif"),
            Cardinal => include_bytes!("../theme/sprites/ic-cardinal.gif"),
            Cburnett => include_bytes!("../theme/sprites/ic-cburnett.gif"),
            Chess7 => include_bytes!("../theme/sprites/ic-chess7.gif"),
            Chessnut => include_bytes!("../theme/sprites/ic-chessnut.gif"),
            Companion => include_bytes!("../theme/sprites/ic-companion.gif"),
            Dubrovny => include_bytes!("../theme/sprites/ic-dubrovny.gif"),
            Fantasy => include_bytes!("../theme/sprites/ic-fantasy.gif"),
            Fresca => include_bytes!("../theme/sprites/ic-fresca.gif"),
            Gioco => include_bytes!("../theme/sprites/ic-gioco.gif"),
            Governor => include_bytes!("../theme/sprites/ic-governor.gif"),
            Horsey => include_bytes!("../theme/sprites/ic-horsey.gif"),
            IcPieces => include_bytes!("../theme/sprites/ic-icpieces.gif"),
            Kosal => include_bytes!("../theme/sprites/ic-kosal.gif"),
            Leipzig => include_bytes!("../theme/sprites/ic-leipzig.gif"),
            Letter => include_bytes!("../theme/sprites/ic-letter.gif"),
            Libra => include_bytes!("../theme/sprites/ic-libra.gif"),
            Maestro => include_bytes!("../theme/sprites/ic-maestro.gif"),
            Merida => include_bytes!("../theme/sprites/ic-merida.gif"),
            Pirouetti => include_bytes!("../theme/sprites/ic-pirouetti.gif"),
            Pixel => include_bytes!("../theme/sprites/ic-pixel.gif"),
            ReillyCraig => include_bytes!("../theme/sprites/ic-reillycraig.gif"),
            Riohacha => include_bytes!("../theme/sprites/ic-riohacha.gif"),
            Shapes => include_bytes!("../theme/sprites/ic-shapes.gif"),
            Spatial => include_bytes!("../theme/sprites/ic-spatial.gif"),
            Staunty => include_bytes!("../theme/sprites/ic-staunty.gif"),
            Tatiana => include_bytes!("../theme/sprites/ic-tatiana.gif"),
        },
        BoardTheme::Purple => match pieces {
            Alpha => include_bytes!("../theme/sprites/purple-alpha.gif"),
            Anarcandy => include_bytes!("../theme/sprites/purple-anarcandy.gif"),
            California => include_bytes!("../theme/sprites/purple-california.gif"),
            Cardinal => include_bytes!("../theme/sprites/purple-cardinal.gif"),
            Cburnett => include_bytes!("../theme/sprites/purple-cburnett.gif"),
            Chess7 => include_bytes!("../theme/sprites/purple-chess7.gif"),
            Chessnut => include_bytes!("../theme/sprites/purple-chessnut.gif"),
            Companion => include_bytes!("../theme/sprites/purple-companion.gif"),
            Dubrovny => include_bytes!("../theme/sprites/purple-dubrovny.gif"),
            Fantasy => include_bytes!("../theme/sprites/purple-fantasy.gif"),
            Fresca => include_bytes!("../theme/sprites/purple-fresca.gif"),
            Gioco => include_bytes!("../theme/sprites/purple-gioco.gif"),
            Governor => include_bytes!("../theme/sprites/purple-governor.gif"),
            Horsey => include_bytes!("../theme/sprites/purple-horsey.gif"),
            IcPieces => include_bytes!("../theme/sprites/purple-icpieces.gif"),
            Kosal => include_bytes!("../theme/sprites/purple-kosal.gif"),
            Leipzig => include_bytes!("../theme/sprites/purple-leipzig.gif"),
            Letter => include_bytes!("../theme/sprites/purple-letter.gif"),
            Libra => include_bytes!("../theme/sprites/purple-libra.gif"),
            Maestro => include_bytes!("../theme/sprites/purple-maestro.gif"),
            Merida => include_bytes!("../theme/sprites/purple-merida.gif"),
            Pirouetti => include_bytes!("../theme/sprites/purple-pirouetti.gif"),
            Pixel => include_bytes!("../theme/sprites/purple-pixel.gif"),
            ReillyCraig => include_bytes!("../theme/sprites/purple-reillycraig.gif"),
            Riohacha => include_bytes!("../theme/sprites/purple-riohacha.gif"),
            Shapes => include_bytes!("../theme/sprites/purple-shapes.gif"),
            Spatial => include_bytes!("../theme/sprites/purple-spatial.gif"),
            Staunty => include_bytes!("../theme/sprites/purple-staunty.gif"),
            Tatiana => include_bytes!("../theme/sprites/purple-tatiana.gif"),
        },
    }
}
