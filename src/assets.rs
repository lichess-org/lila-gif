use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BoardTheme {
    #[serde(alias = "blue2")]
    #[serde(alias = "blue3")]
    #[serde(alias = "blue-marble")]
    #[serde(alias = "canvas")]
    Blue,
    #[serde(alias = "wood")]
    #[serde(alias = "wood2")]
    #[serde(alias = "wood3")]
    #[serde(alias = "wood4")]
    #[serde(alias = "maple")]
    #[serde(alias = "maple2")]
    #[serde(alias = "marble")]
    #[serde(alias = "grey")]
    #[serde(alias = "metal")]
    #[serde(alias = "olive")]
    #[serde(alias = "newspaper")]
    #[serde(alias = "horsey")]
    #[default]
    Brown,
    #[serde(alias = "green-plastic")]
    Green,
    #[serde(alias = "leather")]
    Ic,
    Pink,
    #[serde(alias = "purple-diag")]
    Purple,
}

pub struct ByBoardTheme<T> {
    inner: [T; 6],
}

impl<T> ByBoardTheme<T> {
    pub fn new<F>(f: F) -> ByBoardTheme<T>
    where
        F: FnMut(BoardTheme) -> T,
    {
        use BoardTheme::*;
        ByBoardTheme {
            inner: [Blue, Brown, Green, Ic, Pink, Purple].map(f),
        }
    }

    pub fn by_board_theme(&self, board: BoardTheme) -> &T {
        &self.inner[board as usize]
    }
}

#[derive(Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PieceSet {
    Alpha,
    Anarcandy,
    Caliente,
    California,
    Cardinal,
    #[default]
    Cburnett,
    Celtic,
    Chess7,
    Chessnut,
    Companion,
    Cooke,
    Disguised,
    Dubrovny,
    Fantasy,
    Firi,
    Fresca,
    Gioco,
    Governor,
    Horsey,
    Icpieces,
    #[serde(rename = "kiwen-suwi")]
    KiwenSuwi,
    Kosal,
    Leipzig,
    Letter,
    Maestro,
    Merida,
    Monarchy,
    Mpchess,
    Pirouetti,
    Pixel,
    Reillycraig,
    Rhosgfx,
    Riohacha,
    Shapes,
    Spatial,
    Staunty,
    Tatiana,
    Xkcd,
}

pub struct ByPieceSet<T> {
    inner: [T; 38],
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
                Caliente,
                California,
                Cardinal,
                Cburnett,
                Celtic,
                Chess7,
                Chessnut,
                Companion,
                Cooke,
                Disguised,
                Dubrovny,
                Fantasy,
                Firi,
                Fresca,
                Gioco,
                Governor,
                Horsey,
                Icpieces,
                KiwenSuwi,
                Kosal,
                Leipzig,
                Letter,
                Maestro,
                Merida,
                Monarchy,
                Mpchess,
                Pirouetti,
                Pixel,
                Reillycraig,
                Rhosgfx,
                Riohacha,
                Shapes,
                Spatial,
                Staunty,
                Tatiana,
                Xkcd,
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
            Caliente => include_bytes!("../theme/sprites/blue-caliente.gif"),
            California => include_bytes!("../theme/sprites/blue-california.gif"),
            Cardinal => include_bytes!("../theme/sprites/blue-cardinal.gif"),
            Cburnett => include_bytes!("../theme/sprites/blue-cburnett.gif"),
            Celtic => include_bytes!("../theme/sprites/blue-celtic.gif"),
            Chess7 => include_bytes!("../theme/sprites/blue-chess7.gif"),
            Chessnut => include_bytes!("../theme/sprites/blue-chessnut.gif"),
            Companion => include_bytes!("../theme/sprites/blue-companion.gif"),
            Cooke => include_bytes!("../theme/sprites/blue-cooke.gif"),
            Disguised => include_bytes!("../theme/sprites/blue-disguised.gif"),
            Dubrovny => include_bytes!("../theme/sprites/blue-dubrovny.gif"),
            Fantasy => include_bytes!("../theme/sprites/blue-fantasy.gif"),
            Firi => include_bytes!("../theme/sprites/blue-firi.gif"),
            Fresca => include_bytes!("../theme/sprites/blue-fresca.gif"),
            Gioco => include_bytes!("../theme/sprites/blue-gioco.gif"),
            Governor => include_bytes!("../theme/sprites/blue-governor.gif"),
            Horsey => include_bytes!("../theme/sprites/blue-horsey.gif"),
            Icpieces => include_bytes!("../theme/sprites/blue-icpieces.gif"),
            KiwenSuwi => include_bytes!("../theme/sprites/blue-kiwen-suwi.gif"),
            Kosal => include_bytes!("../theme/sprites/blue-kosal.gif"),
            Leipzig => include_bytes!("../theme/sprites/blue-leipzig.gif"),
            Letter => include_bytes!("../theme/sprites/blue-letter.gif"),
            Maestro => include_bytes!("../theme/sprites/blue-maestro.gif"),
            Merida => include_bytes!("../theme/sprites/blue-merida.gif"),
            Monarchy => include_bytes!("../theme/sprites/blue-monarchy.gif"),
            Mpchess => include_bytes!("../theme/sprites/blue-mpchess.gif"),
            Pirouetti => include_bytes!("../theme/sprites/blue-pirouetti.gif"),
            Pixel => include_bytes!("../theme/sprites/blue-pixel.gif"),
            Reillycraig => include_bytes!("../theme/sprites/blue-reillycraig.gif"),
            Rhosgfx => include_bytes!("../theme/sprites/blue-rhosgfx.gif"),
            Riohacha => include_bytes!("../theme/sprites/blue-riohacha.gif"),
            Shapes => include_bytes!("../theme/sprites/blue-shapes.gif"),
            Spatial => include_bytes!("../theme/sprites/blue-spatial.gif"),
            Staunty => include_bytes!("../theme/sprites/blue-staunty.gif"),
            Tatiana => include_bytes!("../theme/sprites/blue-tatiana.gif"),
            Xkcd => include_bytes!("../theme/sprites/blue-xkcd.gif"),
        },

        BoardTheme::Brown => match pieces {
            Alpha => include_bytes!("../theme/sprites/brown-alpha.gif"),
            Anarcandy => include_bytes!("../theme/sprites/brown-anarcandy.gif"),
            Caliente => include_bytes!("../theme/sprites/brown-caliente.gif"),
            California => include_bytes!("../theme/sprites/brown-california.gif"),
            Cardinal => include_bytes!("../theme/sprites/brown-cardinal.gif"),
            Cburnett => include_bytes!("../theme/sprites/brown-cburnett.gif"),
            Celtic => include_bytes!("../theme/sprites/brown-celtic.gif"),
            Chess7 => include_bytes!("../theme/sprites/brown-chess7.gif"),
            Chessnut => include_bytes!("../theme/sprites/brown-chessnut.gif"),
            Companion => include_bytes!("../theme/sprites/brown-companion.gif"),
            Cooke => include_bytes!("../theme/sprites/brown-cooke.gif"),
            Disguised => include_bytes!("../theme/sprites/brown-disguised.gif"),
            Dubrovny => include_bytes!("../theme/sprites/brown-dubrovny.gif"),
            Fantasy => include_bytes!("../theme/sprites/brown-fantasy.gif"),
            Firi => include_bytes!("../theme/sprites/brown-firi.gif"),
            Fresca => include_bytes!("../theme/sprites/brown-fresca.gif"),
            Gioco => include_bytes!("../theme/sprites/brown-gioco.gif"),
            Governor => include_bytes!("../theme/sprites/brown-governor.gif"),
            Horsey => include_bytes!("../theme/sprites/brown-horsey.gif"),
            Icpieces => include_bytes!("../theme/sprites/brown-icpieces.gif"),
            KiwenSuwi => include_bytes!("../theme/sprites/brown-kiwen-suwi.gif"),
            Kosal => include_bytes!("../theme/sprites/brown-kosal.gif"),
            Leipzig => include_bytes!("../theme/sprites/brown-leipzig.gif"),
            Letter => include_bytes!("../theme/sprites/brown-letter.gif"),
            Maestro => include_bytes!("../theme/sprites/brown-maestro.gif"),
            Merida => include_bytes!("../theme/sprites/brown-merida.gif"),
            Monarchy => include_bytes!("../theme/sprites/brown-monarchy.gif"),
            Mpchess => include_bytes!("../theme/sprites/brown-mpchess.gif"),
            Pirouetti => include_bytes!("../theme/sprites/brown-pirouetti.gif"),
            Pixel => include_bytes!("../theme/sprites/brown-pixel.gif"),
            Reillycraig => include_bytes!("../theme/sprites/brown-reillycraig.gif"),
            Rhosgfx => include_bytes!("../theme/sprites/brown-rhosgfx.gif"),
            Riohacha => include_bytes!("../theme/sprites/brown-riohacha.gif"),
            Shapes => include_bytes!("../theme/sprites/brown-shapes.gif"),
            Spatial => include_bytes!("../theme/sprites/brown-spatial.gif"),
            Staunty => include_bytes!("../theme/sprites/brown-staunty.gif"),
            Tatiana => include_bytes!("../theme/sprites/brown-tatiana.gif"),
            Xkcd => include_bytes!("../theme/sprites/brown-xkcd.gif"),
        },

        BoardTheme::Green => match pieces {
            Alpha => include_bytes!("../theme/sprites/green-alpha.gif"),
            Anarcandy => include_bytes!("../theme/sprites/green-anarcandy.gif"),
            Caliente => include_bytes!("../theme/sprites/green-caliente.gif"),
            California => include_bytes!("../theme/sprites/green-california.gif"),
            Cardinal => include_bytes!("../theme/sprites/green-cardinal.gif"),
            Cburnett => include_bytes!("../theme/sprites/green-cburnett.gif"),
            Celtic => include_bytes!("../theme/sprites/green-celtic.gif"),
            Chess7 => include_bytes!("../theme/sprites/green-chess7.gif"),
            Chessnut => include_bytes!("../theme/sprites/green-chessnut.gif"),
            Companion => include_bytes!("../theme/sprites/green-companion.gif"),
            Cooke => include_bytes!("../theme/sprites/green-cooke.gif"),
            Disguised => include_bytes!("../theme/sprites/green-disguised.gif"),
            Dubrovny => include_bytes!("../theme/sprites/green-dubrovny.gif"),
            Fantasy => include_bytes!("../theme/sprites/green-fantasy.gif"),
            Firi => include_bytes!("../theme/sprites/green-firi.gif"),
            Fresca => include_bytes!("../theme/sprites/green-fresca.gif"),
            Gioco => include_bytes!("../theme/sprites/green-gioco.gif"),
            Governor => include_bytes!("../theme/sprites/green-governor.gif"),
            Horsey => include_bytes!("../theme/sprites/green-horsey.gif"),
            Icpieces => include_bytes!("../theme/sprites/green-icpieces.gif"),
            KiwenSuwi => include_bytes!("../theme/sprites/green-kiwen-suwi.gif"),
            Kosal => include_bytes!("../theme/sprites/green-kosal.gif"),
            Leipzig => include_bytes!("../theme/sprites/green-leipzig.gif"),
            Letter => include_bytes!("../theme/sprites/green-letter.gif"),
            Maestro => include_bytes!("../theme/sprites/green-maestro.gif"),
            Merida => include_bytes!("../theme/sprites/green-merida.gif"),
            Monarchy => include_bytes!("../theme/sprites/green-monarchy.gif"),
            Mpchess => include_bytes!("../theme/sprites/green-mpchess.gif"),
            Pirouetti => include_bytes!("../theme/sprites/green-pirouetti.gif"),
            Pixel => include_bytes!("../theme/sprites/green-pixel.gif"),
            Reillycraig => include_bytes!("../theme/sprites/green-reillycraig.gif"),
            Rhosgfx => include_bytes!("../theme/sprites/green-rhosgfx.gif"),
            Riohacha => include_bytes!("../theme/sprites/green-riohacha.gif"),
            Shapes => include_bytes!("../theme/sprites/green-shapes.gif"),
            Spatial => include_bytes!("../theme/sprites/green-spatial.gif"),
            Staunty => include_bytes!("../theme/sprites/green-staunty.gif"),
            Tatiana => include_bytes!("../theme/sprites/green-tatiana.gif"),
            Xkcd => include_bytes!("../theme/sprites/green-xkcd.gif"),
        },

        BoardTheme::Ic => match pieces {
            Alpha => include_bytes!("../theme/sprites/ic-alpha.gif"),
            Anarcandy => include_bytes!("../theme/sprites/ic-anarcandy.gif"),
            Caliente => include_bytes!("../theme/sprites/ic-caliente.gif"),
            California => include_bytes!("../theme/sprites/ic-california.gif"),
            Cardinal => include_bytes!("../theme/sprites/ic-cardinal.gif"),
            Cburnett => include_bytes!("../theme/sprites/ic-cburnett.gif"),
            Celtic => include_bytes!("../theme/sprites/ic-celtic.gif"),
            Chess7 => include_bytes!("../theme/sprites/ic-chess7.gif"),
            Chessnut => include_bytes!("../theme/sprites/ic-chessnut.gif"),
            Companion => include_bytes!("../theme/sprites/ic-companion.gif"),
            Cooke => include_bytes!("../theme/sprites/ic-cooke.gif"),
            Disguised => include_bytes!("../theme/sprites/ic-disguised.gif"),
            Dubrovny => include_bytes!("../theme/sprites/ic-dubrovny.gif"),
            Fantasy => include_bytes!("../theme/sprites/ic-fantasy.gif"),
            Firi => include_bytes!("../theme/sprites/ic-firi.gif"),
            Fresca => include_bytes!("../theme/sprites/ic-fresca.gif"),
            Gioco => include_bytes!("../theme/sprites/ic-gioco.gif"),
            Governor => include_bytes!("../theme/sprites/ic-governor.gif"),
            Horsey => include_bytes!("../theme/sprites/ic-horsey.gif"),
            Icpieces => include_bytes!("../theme/sprites/ic-icpieces.gif"),
            KiwenSuwi => include_bytes!("../theme/sprites/ic-kiwen-suwi.gif"),
            Kosal => include_bytes!("../theme/sprites/ic-kosal.gif"),
            Leipzig => include_bytes!("../theme/sprites/ic-leipzig.gif"),
            Letter => include_bytes!("../theme/sprites/ic-letter.gif"),
            Maestro => include_bytes!("../theme/sprites/ic-maestro.gif"),
            Merida => include_bytes!("../theme/sprites/ic-merida.gif"),
            Monarchy => include_bytes!("../theme/sprites/ic-monarchy.gif"),
            Mpchess => include_bytes!("../theme/sprites/ic-mpchess.gif"),
            Pirouetti => include_bytes!("../theme/sprites/ic-pirouetti.gif"),
            Pixel => include_bytes!("../theme/sprites/ic-pixel.gif"),
            Reillycraig => include_bytes!("../theme/sprites/ic-reillycraig.gif"),
            Rhosgfx => include_bytes!("../theme/sprites/ic-rhosgfx.gif"),
            Riohacha => include_bytes!("../theme/sprites/ic-riohacha.gif"),
            Shapes => include_bytes!("../theme/sprites/ic-shapes.gif"),
            Spatial => include_bytes!("../theme/sprites/ic-spatial.gif"),
            Staunty => include_bytes!("../theme/sprites/ic-staunty.gif"),
            Tatiana => include_bytes!("../theme/sprites/ic-tatiana.gif"),
            Xkcd => include_bytes!("../theme/sprites/ic-xkcd.gif"),
        },

        BoardTheme::Pink => match pieces {
            Alpha => include_bytes!("../theme/sprites/pink-alpha.gif"),
            Anarcandy => include_bytes!("../theme/sprites/pink-anarcandy.gif"),
            Caliente => include_bytes!("../theme/sprites/pink-caliente.gif"),
            California => include_bytes!("../theme/sprites/pink-california.gif"),
            Cardinal => include_bytes!("../theme/sprites/pink-cardinal.gif"),
            Cburnett => include_bytes!("../theme/sprites/pink-cburnett.gif"),
            Celtic => include_bytes!("../theme/sprites/pink-celtic.gif"),
            Chess7 => include_bytes!("../theme/sprites/pink-chess7.gif"),
            Chessnut => include_bytes!("../theme/sprites/pink-chessnut.gif"),
            Companion => include_bytes!("../theme/sprites/pink-companion.gif"),
            Cooke => include_bytes!("../theme/sprites/pink-cooke.gif"),
            Disguised => include_bytes!("../theme/sprites/pink-disguised.gif"),
            Dubrovny => include_bytes!("../theme/sprites/pink-dubrovny.gif"),
            Fantasy => include_bytes!("../theme/sprites/pink-fantasy.gif"),
            Firi => include_bytes!("../theme/sprites/pink-firi.gif"),
            Fresca => include_bytes!("../theme/sprites/pink-fresca.gif"),
            Gioco => include_bytes!("../theme/sprites/pink-gioco.gif"),
            Governor => include_bytes!("../theme/sprites/pink-governor.gif"),
            Horsey => include_bytes!("../theme/sprites/pink-horsey.gif"),
            Icpieces => include_bytes!("../theme/sprites/pink-icpieces.gif"),
            KiwenSuwi => include_bytes!("../theme/sprites/pink-kiwen-suwi.gif"),
            Kosal => include_bytes!("../theme/sprites/pink-kosal.gif"),
            Leipzig => include_bytes!("../theme/sprites/pink-leipzig.gif"),
            Letter => include_bytes!("../theme/sprites/pink-letter.gif"),
            Maestro => include_bytes!("../theme/sprites/pink-maestro.gif"),
            Merida => include_bytes!("../theme/sprites/pink-merida.gif"),
            Monarchy => include_bytes!("../theme/sprites/pink-monarchy.gif"),
            Mpchess => include_bytes!("../theme/sprites/pink-mpchess.gif"),
            Pirouetti => include_bytes!("../theme/sprites/pink-pirouetti.gif"),
            Pixel => include_bytes!("../theme/sprites/pink-pixel.gif"),
            Reillycraig => include_bytes!("../theme/sprites/pink-reillycraig.gif"),
            Rhosgfx => include_bytes!("../theme/sprites/pink-rhosgfx.gif"),
            Riohacha => include_bytes!("../theme/sprites/pink-riohacha.gif"),
            Shapes => include_bytes!("../theme/sprites/pink-shapes.gif"),
            Spatial => include_bytes!("../theme/sprites/pink-spatial.gif"),
            Staunty => include_bytes!("../theme/sprites/pink-staunty.gif"),
            Tatiana => include_bytes!("../theme/sprites/pink-tatiana.gif"),
            Xkcd => include_bytes!("../theme/sprites/pink-xkcd.gif"),
        },

        BoardTheme::Purple => match pieces {
            Alpha => include_bytes!("../theme/sprites/purple-alpha.gif"),
            Anarcandy => include_bytes!("../theme/sprites/purple-anarcandy.gif"),
            Caliente => include_bytes!("../theme/sprites/purple-caliente.gif"),
            California => include_bytes!("../theme/sprites/purple-california.gif"),
            Cardinal => include_bytes!("../theme/sprites/purple-cardinal.gif"),
            Cburnett => include_bytes!("../theme/sprites/purple-cburnett.gif"),
            Celtic => include_bytes!("../theme/sprites/purple-celtic.gif"),
            Chess7 => include_bytes!("../theme/sprites/purple-chess7.gif"),
            Chessnut => include_bytes!("../theme/sprites/purple-chessnut.gif"),
            Companion => include_bytes!("../theme/sprites/purple-companion.gif"),
            Cooke => include_bytes!("../theme/sprites/purple-cooke.gif"),
            Disguised => include_bytes!("../theme/sprites/purple-disguised.gif"),
            Dubrovny => include_bytes!("../theme/sprites/purple-dubrovny.gif"),
            Fantasy => include_bytes!("../theme/sprites/purple-fantasy.gif"),
            Firi => include_bytes!("../theme/sprites/purple-firi.gif"),
            Fresca => include_bytes!("../theme/sprites/purple-fresca.gif"),
            Gioco => include_bytes!("../theme/sprites/purple-gioco.gif"),
            Governor => include_bytes!("../theme/sprites/purple-governor.gif"),
            Horsey => include_bytes!("../theme/sprites/purple-horsey.gif"),
            Icpieces => include_bytes!("../theme/sprites/purple-icpieces.gif"),
            KiwenSuwi => include_bytes!("../theme/sprites/purple-kiwen-suwi.gif"),
            Kosal => include_bytes!("../theme/sprites/purple-kosal.gif"),
            Leipzig => include_bytes!("../theme/sprites/purple-leipzig.gif"),
            Letter => include_bytes!("../theme/sprites/purple-letter.gif"),
            Maestro => include_bytes!("../theme/sprites/purple-maestro.gif"),
            Merida => include_bytes!("../theme/sprites/purple-merida.gif"),
            Monarchy => include_bytes!("../theme/sprites/purple-monarchy.gif"),
            Mpchess => include_bytes!("../theme/sprites/purple-mpchess.gif"),
            Pirouetti => include_bytes!("../theme/sprites/purple-pirouetti.gif"),
            Pixel => include_bytes!("../theme/sprites/purple-pixel.gif"),
            Reillycraig => include_bytes!("../theme/sprites/purple-reillycraig.gif"),
            Rhosgfx => include_bytes!("../theme/sprites/purple-rhosgfx.gif"),
            Riohacha => include_bytes!("../theme/sprites/purple-riohacha.gif"),
            Shapes => include_bytes!("../theme/sprites/purple-shapes.gif"),
            Spatial => include_bytes!("../theme/sprites/purple-spatial.gif"),
            Staunty => include_bytes!("../theme/sprites/purple-staunty.gif"),
            Tatiana => include_bytes!("../theme/sprites/purple-tatiana.gif"),
            Xkcd => include_bytes!("../theme/sprites/purple-xkcd.gif"),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::convert::identity;

    use super::*;

    #[test]
    fn test_by_piece_set() {
        assert_eq!(
            ByPieceSet::new(identity).by_piece_set(PieceSet::Tatiana),
            &PieceSet::Tatiana
        );
    }
}
