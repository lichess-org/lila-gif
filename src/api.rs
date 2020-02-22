use serde::Deserialize;
use serde::de::Deserializer;
use serde_with::rust::display_fromstr;
use shakmaty::Square;
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;

#[derive(Deserialize)]
struct RequestBody {
    white: String,
    black: String,
    frames: Vec<RequestFrame>,
    #[serde(default)]
    flipped: bool,
}

#[derive(Deserialize)]
struct RequestFrame {
    #[serde(with = "display_fromstr")]
    f: Fen,
    #[serde(default)]
    d: u16,
    #[serde(deserialize_with = "display_fromstr::deserialize", default = "uci_null")]
    m: Uci,
    #[serde(deserialize_with = "maybe_square")]
    check: Option<Square>,
}

fn uci_null() -> Uci {
    Uci::Null
}

fn maybe_square<'de, D>(deserializer: D) -> Result<Option<Square>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<&str>::deserialize(deserializer).and_then(|maybe_name| {
        Ok(match maybe_name {
            Some(name) => Some(name.parse().map_err(|_| serde::de::Error::custom("invald square name"))?),
            None => None,
        })
    })
}
