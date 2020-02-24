use arrayvec::ArrayString;
use serde::{de, Deserialize};
use serde_with::rust::display_fromstr;
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;
use shakmaty::Square;

#[derive(Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum Orientation {
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation::White
    }
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

pub type PlayerName = ArrayString<[u8; 100]>;

#[derive(Deserialize)]
pub struct RequestParams {
    pub white: Option<PlayerName>,
    pub black: Option<PlayerName>,
    #[serde(with = "display_fromstr", default)]
    pub fen: Fen,
    #[serde(deserialize_with = "display_fromstr::deserialize", default = "uci_null", rename = "lastMove")]
    pub last_move: Uci,
    #[serde(deserialize_with = "maybe_square", default)]
    pub check: Option<Square>,
    #[serde(default)]
    pub orientation: Orientation,
}

#[derive(Deserialize)]
pub struct RequestBody {
    pub white: Option<PlayerName>,
    pub black: Option<PlayerName>,
    pub frames: Vec<RequestFrame>,
    #[serde(default)]
    pub orientation: Orientation,
    #[serde(default)]
    pub delay: u16,
}

#[derive(Deserialize)]
pub struct RequestFrame {
    #[serde(with = "display_fromstr")]
    pub fen: Fen,
    pub delay: Option<u16>,
    #[serde(deserialize_with = "display_fromstr::deserialize", default = "uci_null", alias = "lastMove")]
    pub last_move: Uci,
    #[serde(deserialize_with = "maybe_square")]
    pub check: Option<Square>,
}

fn uci_null() -> Uci {
    Uci::Null
}

fn maybe_square<'de, D>(deserializer: D) -> Result<Option<Square>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Option::<&str>::deserialize(deserializer).and_then(|maybe_name| {
        Ok(match maybe_name {
            Some(name) => Some(name.parse().map_err(|_| de::Error::custom("invalid square name"))?),
            None => None,
        })
    })
}

impl RequestBody {
    pub fn example() -> RequestBody {
        RequestBody {
            white: Some(PlayerName::from("Molinari").unwrap()),
            black: Some(PlayerName::from("Bordais").unwrap()),
            orientation: Orientation::White,
            delay: 50,
            frames: vec![
                RequestFrame {
                    fen: Fen::default(),
                    delay: None,
                    last_move: Uci::Null,
                    check: None,
                },
                RequestFrame {
                    fen: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".parse().unwrap(),
                    delay: None,
                    last_move: "e2e4".parse().unwrap(),
                    check: None,
                },
                RequestFrame {
                    fen: "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".parse().unwrap(),
                    delay: None,
                    last_move: "c7c5".parse().unwrap(),
                    check: None,
                },
                RequestFrame {
                    fen: "rnbqkbnr/pp1ppppp/8/2p5/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 2".parse().unwrap(),
                    delay: None,
                    last_move: "c2c4".parse().unwrap(),
                    check: None,
                },
                RequestFrame {
                    fen: "r1bqkbnr/pp1ppppp/2n5/2p5/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 1 3".parse().unwrap(),
                    delay: None,
                    last_move: "b8c6".parse().unwrap(),
                    check: None,
                },
                RequestFrame {
                    fen: "r1bqkbnr/pp1ppppp/2n5/2p5/2P1P3/8/PP1PNPPP/RNBQKB1R b KQkq - 2 3".parse().unwrap(),
                    delay: None,
                    last_move: "g1e2".parse().unwrap(),
                    check: None,
                },
                RequestFrame {
                    fen: "r1bqkb1r/pp1ppppp/2n2n2/2p5/2P1P3/8/PP1PNPPP/RNBQKB1R w KQkq - 3 4".parse().unwrap(),
                    delay: None,
                    last_move: "g8f6".parse().unwrap(),
                    check: None,
                },
                RequestFrame {
                    fen: "r1bqkb1r/pp1ppppp/2n2n2/2p5/2P1P3/2N5/PP1PNPPP/R1BQKB1R b KQkq - 4 4".parse().unwrap(),
                    delay: None,
                    last_move: "b1c3".parse().unwrap(),
                    check: None,
                },
                RequestFrame {
                    fen: "r1bqkb1r/pp1ppppp/5n2/2p5/1nP1P3/2N5/PP1PNPPP/R1BQKB1R w KQkq - 5 5".parse().unwrap(),
                    delay: None,
                    last_move: "c6b4".parse().unwrap(),
                    check: None,
                },
                RequestFrame {
                    fen: "r1bqkb1r/pp1ppppp/5n2/2p5/1nP1P3/2N3P1/PP1PNP1P/R1BQKB1R b KQkq - 0 5".parse().unwrap(),
                    delay: None,
                    last_move: "g2g3".parse().unwrap(),
                    check: None,
                },
                RequestFrame {
                    fen: "r1bqkb1r/pp1ppppp/5n2/2p5/2P1P3/2Nn2P1/PP1PNP1P/R1BQKB1R w KQkq - 1 6".parse().unwrap(),
                    delay: Some(500),
                    last_move: "b4d3".parse().unwrap(),
                    check: Some(Square::E1),
                },
            ],
        }
    }
}
