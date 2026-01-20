use std::fmt;

use arrayvec::ArrayString;
use serde::{de, Deserialize};
use serde_with::{serde_as, DisplayFromStr};
use shakmaty::{
    fen::Fen, san::San, uci::UciMove, CastlingMode, Chess, EnPassantMode, Position, Setup, Square,
};

use crate::assets::{BoardTheme, PieceSet};

#[derive(Deserialize, Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Orientation {
    #[serde(rename = "white")]
    #[default]
    White,
    #[serde(rename = "black")]
    Black,
}

impl Orientation {
    pub fn fold<T>(self, white: T, black: T) -> T {
        match self {
            Orientation::White => white,
            Orientation::Black => black,
        }
    }

    pub fn x(self, square: Square) -> usize {
        self.fold(usize::from(square.file()), 7 - usize::from(square.file()))
    }

    pub fn y(self, square: Square) -> usize {
        self.fold(7 - usize::from(square.rank()), usize::from(square.rank()))
    }
}

pub type PlayerName = ArrayString<100>; // length limited to prevent dos

pub type Comment = ArrayString<255>; // strict length limit for gif comments

#[derive(Debug, Default, Copy, Clone)]
pub enum CheckSquare {
    #[default]
    No,
    Yes,
    Square(Square),
}

impl<'de> Deserialize<'de> for CheckSquare {
    fn deserialize<D>(deseralizer: D) -> Result<CheckSquare, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct CheckSquareVisitor;

        impl de::Visitor<'_> for CheckSquareVisitor {
            type Value = CheckSquare;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("square name or bool")
            }

            fn visit_str<E>(self, name: &str) -> Result<CheckSquare, E>
            where
                E: de::Error,
            {
                if name == "1" || name == "yes" || name == "true" {
                    Ok(CheckSquare::Yes)
                } else if name == "0" || name == "no" || name == "false" {
                    Ok(CheckSquare::No)
                } else {
                    match name.parse() {
                        Ok(sq) => Ok(CheckSquare::Square(sq)),
                        Err(_) => Err(de::Error::custom("invalid square name")),
                    }
                }
            }

            fn visit_bool<E>(self, yes: bool) -> Result<CheckSquare, E>
            where
                E: de::Error,
            {
                Ok(match yes {
                    true => CheckSquare::Yes,
                    false => CheckSquare::No,
                })
            }
        }

        deseralizer.deserialize_any(CheckSquareVisitor)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Coordinates {
    No,
    #[default]
    Yes,
}

impl<'de> Deserialize<'de> for Coordinates {
    fn deserialize<D>(deseralizer: D) -> Result<Coordinates, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct CoordinatesVisitor;

        impl de::Visitor<'_> for CoordinatesVisitor {
            type Value = Coordinates;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("\"1\", \"yes\", \"true\", \"0\", \"no\", \"false\" or bool")
            }

            fn visit_str<E>(self, name: &str) -> Result<Coordinates, E>
            where
                E: de::Error,
            {
                if name == "1" || name == "yes" || name == "true" {
                    Ok(Coordinates::Yes)
                } else if name == "0" || name == "no" || name == "false" {
                    Ok(Coordinates::No)
                } else {
                    Err(de::Error::custom("invalid coordinates value"))
                }
            }

            fn visit_bool<E>(self, yes: bool) -> Result<Coordinates, E>
            where
                E: de::Error,
            {
                Ok(match yes {
                    true => Coordinates::Yes,
                    false => Coordinates::No,
                })
            }
        }

        deseralizer.deserialize_any(CoordinatesVisitor)
    }
}

impl CheckSquare {
    pub fn to_square(self, setup: &Setup) -> Option<Square> {
        match self {
            CheckSquare::No => None,
            CheckSquare::Yes => setup.board.king_of(setup.turn),
            CheckSquare::Square(sq) => Some(sq),
        }
    }
}

#[derive(Deserialize, Default)]
pub struct FrameClock {
    pub white: Option<u32>,
    pub black: Option<u32>,
}

#[derive(Copy, Clone, strum::EnumIter, strum::EnumString, strum::IntoStaticStr)]
#[repr(u8)]
pub enum MoveGlyph {
    #[strum(serialize = "!")]
    Good = 1,
    #[strum(serialize = "!!")]
    Brilliant,
    #[strum(serialize = "?")]
    Mistake,
    #[strum(serialize = "??")]
    Blunder,
    #[strum(serialize = "!?")]
    Interesting,
    #[strum(serialize = "?!")]
    Dubious,
    #[strum(serialize = "□")]
    OnlyMove,
    #[strum(serialize = "⨀", to_string = "O")]
    Zugzwang,
}

impl MoveGlyph {
    pub const fn index(self) -> u8 {
        self as u8
    }

    pub const fn color(self) -> [u8; 3] {
        match self {
            Self::Good => [0x22, 0xac, 0x38],        // green
            Self::Brilliant => [0x16, 0x82, 0x26],   // dark green
            Self::Mistake => [0xe6, 0x9f, 0x00],     // orange
            Self::Blunder => [0xdf, 0x53, 0x53],     // red
            Self::Interesting => [0xea, 0x45, 0xd8], // pink/magenta
            Self::Dubious => [0x56, 0xb4, 0xe9],     // light blue
            Self::OnlyMove => [0xa0, 0x40, 0x48],    // maroon
            Self::Zugzwang => [0x91, 0x71, 0xf2],    // purple
        }
    }
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct RequestParams {
    pub white: Option<PlayerName>,
    pub black: Option<PlayerName>,
    pub comment: Option<Comment>,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub fen: Fen,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default, rename = "lastMove")]
    pub last_move: Option<UciMove>,
    #[serde(default)]
    pub check: CheckSquare,
    #[serde(default)]
    pub orientation: Orientation,
    #[serde(default)]
    pub theme: BoardTheme,
    #[serde(default)]
    pub piece: PieceSet,
    #[serde(default)]
    pub coordinates: Coordinates,
}

#[derive(Deserialize)]
pub struct RequestBody {
    pub white: Option<PlayerName>,
    pub black: Option<PlayerName>,
    pub comment: Option<Comment>,
    pub frames: Vec<RequestFrame>,
    #[serde(default)]
    pub orientation: Orientation,
    #[serde(default)]
    pub delay: u16,
    #[serde(default)]
    pub theme: BoardTheme,
    #[serde(default)]
    pub piece: PieceSet,
    #[serde(default)]
    pub coordinates: Coordinates,
}

#[serde_as]
#[derive(Deserialize, Default)]
pub struct RequestFrame {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub fen: Fen,
    #[serde(default)]
    pub delay: Option<u16>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default, rename = "lastMove")]
    pub last_move: Option<UciMove>,
    #[serde(default)]
    pub check: CheckSquare,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub glyph: Option<MoveGlyph>,
    #[serde(default)]
    pub clock: FrameClock,
}

impl RequestBody {
    pub fn example() -> RequestBody {
        let pgn = "\
            1. c4 Nf6 2. Nc3 e5 3. d4 exd4 4. Qxd4 Nc6 5. Qd1 Bb4 6. Bd2 O-O \
            7. e3 Bxc3 8. Bxc3 Ne4 9. Ne2 d6 10. Qc2 Re8 11. Nf4 Bf5 \
            12. Bd3 Qg5 13. O-O g6 14. Rae1 Nxc3 15. Qxc3 Bxd3 16. Qxd3 Ne5 \
            17. Qd1 Rad8 18. b3 c6 19. Kh1 Qf5 20. Qa1 h5 21. Rd1 Ng4 \
            22. h3 Nf6 23. Qd4 a6 24. f3 Qe5 25. Rfe1 Qxd4 26. exd4 Rxe1+ \
            27. Rxe1 Kf8 28. h4 Re8 29. Rxe8+ Kxe8 30. Kg1 Ng8 31. Kf2 Nh6 \
            32. g3 Ke7 33. Ng2 Ke6 34. Ne3 a5 35. g4 f6 36. Kg3 d5 37. c5 Nf7 \
            38. Ng2 hxg4 39. fxg4 Nd8 40. Nf4+ Kf7 41. h5 g5 42. Ne2 Ne6 \
            43. Kf3 Kg7 44. Ke3 Kh6 45. Ng3 Ng7 46. Nf5+ Nxf5+";

        let mut frames = Vec::with_capacity(46 * 2 + 1);
        frames.push(RequestFrame::default());

        let mut pos = Chess::default();
        for pgn_move in pgn.split(' ') {
            if pgn_move.trim().is_empty() || pgn_move.ends_with('.') {
                continue;
            }

            let san: San = pgn_move.parse().unwrap();
            let m = san.to_move(&pos).unwrap();
            pos.play_unchecked(m);

            frames.push(RequestFrame {
                fen: Fen::from_position(&pos, EnPassantMode::Always),
                check: if pos.is_check() {
                    CheckSquare::Yes
                } else {
                    CheckSquare::No
                },
                last_move: Some(UciMove::from_move(m, CastlingMode::Standard)),
                delay: None,
                glyph: None,
                clock: FrameClock::default(),
            })
        }

        frames.last_mut().unwrap().delay = Some(500);

        RequestBody {
            comment: Some(Comment::from("https://lichess.org/Q0iQs5Zi").unwrap()),
            white: Some(PlayerName::from("GM DrDrunkenstein (2888)").unwrap()),
            black: Some(PlayerName::from("GM Zhigalko_Sergei (2895)").unwrap()),
            orientation: Orientation::White,
            delay: 50,
            frames,
            theme: BoardTheme::default(),
            piece: PieceSet::default(),
            coordinates: Coordinates::default(),
        }
    }
}
