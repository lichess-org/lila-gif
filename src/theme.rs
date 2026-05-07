use gift::block::{ColorTableConfig, GlobalColorTable};
use ndarray::{Array2, ArrayView2, s};
use rusttype::Font;
use shakmaty::{Piece, Role};

use crate::{
    api::MoveGlyph,
    assets::{BoardTheme, ByBoardTheme, ByPieceSet, PieceSet, sprite_data},
};

const SQUARE: usize = 90;

pub enum Sprite<'a> {
    Paste(ArrayView2<'a, u8>),
    Fill(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Gradient {
    LightDark = 7,
    LightHighlightDarkHighlight = 8,
    TransparentTransparent = 9,
    TextBar = 10,
    GoldBar = 11,
    BotBar = 12,
    GlyphGood = 13,
    GlyphBrilliant = 14,
    GlyphMistake = 15,
    GlyphBlunder = 16,
    GlyphInteresting = 17,
    GlyphDubious = 18,
    GlyphOnlyMove = 19,
    GlyphZugzwang = 20,
}

impl From<MoveGlyph> for Gradient {
    fn from(glyph: MoveGlyph) -> Self {
        match glyph {
            MoveGlyph::Good => Gradient::GlyphGood,
            MoveGlyph::Brilliant => Gradient::GlyphBrilliant,
            MoveGlyph::Mistake => Gradient::GlyphMistake,
            MoveGlyph::Blunder => Gradient::GlyphBlunder,
            MoveGlyph::Interesting => Gradient::GlyphInteresting,
            MoveGlyph::Dubious => Gradient::GlyphDubious,
            MoveGlyph::OnlyMove => Gradient::GlyphOnlyMove,
            MoveGlyph::Zugzwang => Gradient::GlyphZugzwang,
        }
    }
}

pub struct SpriteKey {
    pub piece: Option<Piece>,
    pub dark_square: bool,
    pub highlight: bool,
    pub check: bool,
}

impl SpriteKey {
    pub fn light_dark_gradient(&self) -> Gradient {
        if self.highlight {
            Gradient::LightHighlightDarkHighlight
        } else {
            Gradient::LightDark
        }
    }
}

pub struct Theme {
    color_table_config: ColorTableConfig,
    global_color_table: GlobalColorTable,
    sprite: Array2<u8>,
}

impl Theme {
    fn new(mut sprite_data: &[u8]) -> Theme {
        let mut decoder = gift::Decoder::new(&mut sprite_data).into_frames();
        let preamble = decoder
            .preamble()
            .expect("decode preamble")
            .expect("preamble");

        let frame = decoder.next().expect("frame").expect("decode frame");
        let sprite = Array2::from_shape_vec(
            (SQUARE * (7 + 14), SQUARE * 8),
            frame.image_data.data().to_owned(),
        )
        .expect("from shape");

        Theme {
            color_table_config: preamble.logical_screen_desc.color_table_config(),
            global_color_table: preamble.global_color_table.expect("sprite has color table"),
            sprite,
        }
    }

    pub fn square(&self) -> usize {
        SQUARE
    }

    pub fn width(&self) -> usize {
        self.square() * 8
    }

    pub fn bar_height(&self) -> usize {
        60
    }

    pub fn height(&self, bars: bool) -> usize {
        if bars {
            self.width() + 2 * self.bar_height()
        } else {
            self.width()
        }
    }

    pub fn color_table_config(&self) -> ColorTableConfig {
        self.color_table_config
    }

    pub fn global_color_table(&self) -> &GlobalColorTable {
        &self.global_color_table
    }

    pub fn gradient_color(&self, gradient: Gradient, intensity: f32) -> u8 {
        let max_x = ((SQUARE * 8) - 1) as f32;
        let x = ((1.0 - intensity.clamp(0.0, 1.0)) * max_x) as usize;
        let y = (gradient as usize) * SQUARE;
        self.sprite[(y, x)]
    }

    pub fn bar_color(&self) -> u8 {
        self.gradient_color(Gradient::TextBar, 0.0)
    }

    pub fn text_color(&self) -> u8 {
        self.gradient_color(Gradient::TextBar, 1.0)
    }

    pub fn med_text_color(&self) -> u8 {
        self.gradient_color(Gradient::TextBar, 0.5)
    }

    pub fn transparent_color(&self) -> u8 {
        self.gradient_color(Gradient::TransparentTransparent, 1.0)
    }

    pub fn glyph_background_color(&self, glyph: MoveGlyph) -> u8 {
        self.gradient_color(Gradient::from(glyph), 0.0)
    }

    pub fn sprite<'a>(&'a self, key: &SpriteKey) -> Sprite<'a> {
        match *key {
            SpriteKey {
                piece: Some(piece),
                dark_square,
                highlight,
                check,
            } => {
                let x = 4 * usize::from(piece.color.is_white())
                    + 2 * usize::from(highlight)
                    + usize::from(dark_square);
                let y = if piece.role == Role::King && check {
                    6
                } else {
                    piece.role as usize - 1
                };
                Sprite::Paste(self.sprite.slice(s!(
                    (SQUARE * y)..(SQUARE + SQUARE * y),
                    (SQUARE * x)..(SQUARE + SQUARE * x)
                )))
            }
            SpriteKey {
                highlight: false,
                dark_square,
                ..
            } => Sprite::Fill(self.gradient_color(Gradient::LightDark, f32::from(!dark_square))),
            SpriteKey {
                highlight: true,
                dark_square,
                ..
            } => Sprite::Fill(self.gradient_color(
                Gradient::LightHighlightDarkHighlight,
                f32::from(!dark_square),
            )),
        }
    }
}

pub struct Themes {
    map: ByBoardTheme<ByPieceSet<Theme>>,
    font: Font<'static>,
}

impl Themes {
    pub fn new() -> Themes {
        let font_data = include_bytes!("../theme/font/NotoSans-Regular.ttf") as &[u8];
        let font = Font::try_from_bytes(font_data).expect("parse font");

        Themes {
            map: ByBoardTheme::new(|board| {
                ByPieceSet::new(|pieces| Theme::new(sprite_data(board, pieces)))
            }),
            font,
        }
    }

    pub fn font(&self) -> &Font<'_> {
        &self.font
    }

    pub fn get(&self, board: BoardTheme, pieces: PieceSet) -> &Theme {
        self.map.by_board_theme(board).by_piece_set(pieces)
    }
}
