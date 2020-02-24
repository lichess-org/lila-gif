use arrayvec::ArrayString;
use serde::{Deserialize, de};
use serde_with::rust::display_fromstr;
use shakmaty::Square;
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;

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

/* #[derive(Deserialize)]
pub struct RequestBody {
    white: Option<String>, // TODO: limit length
    black: Option<String>,
    frames: Vec<RequestFrame>,
    #[serde(default)]
    flipped: bool,
    #[serde(default)]
    delay: u16,
}

#[derive(Deserialize)]
pub struct RequestFrame {
    #[serde(with = "display_fromstr")]
    fen: Fen,
    delay: Option<u16>,
    #[serde(deserialize_with = "display_fromstr::deserialize", default = "uci_null", alias = "lastMove")]
    last_move: Uci,
    #[serde(deserialize_with = "maybe_square")]
    check: Option<Square>,
} */

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
