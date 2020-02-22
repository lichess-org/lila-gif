use serde::{Deserialize, de};
use serde_with::rust::display_fromstr;
use shakmaty::{Color, Square};
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;

#[derive(Deserialize)]
pub struct RequestBody {
    white: Option<String>,
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
            Some(name) => Some(name.parse().map_err(|_| de::Error::custom("invald square name"))?),
            None => None,
        })
    })
}
