use gift::block::{ColorTableConfig, ColorTableExistence, ColorTableOrdering, GlobalColorTable};
use ndarray::{s, Array2, ArrayView2};
use rusttype::Font;
use shakmaty::{Piece, Role};

use crate::{
    api::MoveGlyph,
    assets::{sprite_data, BoardTheme, ByBoardTheme, ByPieceSet, PieceSet},
};

const SQUARE: usize = 90;
const COLOR_WIDTH: usize = 90 * 2 / 3;
const GLYPH_TEXT_COLOR: [u8; 3] = [0xff, 0xff, 0xff];

pub struct SpriteKey {
    pub piece: Option<Piece>,
    pub dark_square: bool,
    pub highlight: bool,
    pub check: bool,
}

impl SpriteKey {
    fn x(&self) -> usize {
        4 * usize::from(self.piece.is_some_and(|p| p.color.is_white()))
            + 2 * usize::from(self.highlight)
            + usize::from(self.dark_square)
    }

    fn y(&self) -> usize {
        match self.piece {
            Some(piece) if self.check && piece.role == Role::King => 7,
            Some(piece) => piece.role as usize,
            None => 0,
        }
    }
}

pub struct Theme {
    color_table_config: ColorTableConfig,
    global_color_table: GlobalColorTable,
    sprite: Array2<u8>,
    extended_color_idx: u8,
}

impl Theme {
    fn new(sprite_data: &[u8]) -> Theme {
        let mut decoder = gift::Decoder::new(std::io::Cursor::new(sprite_data)).into_frames();
        let preamble = decoder
            .preamble()
            .expect("decode preamble")
            .expect("preamble");
        let frame = decoder.next().expect("frame").expect("decode frame");
        let sprite =
            Array2::from_shape_vec((SQUARE * 8, SQUARE * 8), frame.image_data.data().to_owned())
                .expect("from shape");
        let mut colors = preamble
            .global_color_table
            .expect("color table present")
            .colors()
            .to_vec();
        let extended_color_idx = (colors.len() / 3) as u8;
        colors.extend_from_slice(&GLYPH_TEXT_COLOR);

        for glyph in MoveGlyph::ALL {
            colors.extend_from_slice(&glyph.color());
        }

        // GIF requires power-of-2 color table sizes
        let padded_count = (colors.len() / 3).next_power_of_two().min(256);
        colors.resize(padded_count * 3, 0);

        Theme {
            color_table_config: ColorTableConfig::new(
                ColorTableExistence::Present,
                ColorTableOrdering::NotSorted,
                padded_count as u16,
            ),
            global_color_table: GlobalColorTable::with_colors(&colors),
            sprite,
            extended_color_idx,
        }
    }

    pub fn color_table_config(&self) -> ColorTableConfig {
        self.color_table_config
    }

    pub fn global_color_table(&self) -> &GlobalColorTable {
        &self.global_color_table
    }

    pub fn bar_color(&self) -> u8 {
        self.sprite[(0, SQUARE * 4)]
    }

    pub fn text_color(&self) -> u8 {
        self.sprite[(0, SQUARE * 4 + COLOR_WIDTH)]
    }

    pub fn gold_color(&self) -> u8 {
        self.sprite[(0, SQUARE * 4 + COLOR_WIDTH * 2)]
    }

    pub fn bot_color(&self) -> u8 {
        self.sprite[(0, SQUARE * 4 + COLOR_WIDTH * 3)]
    }

    pub fn med_text_color(&self) -> u8 {
        self.sprite[(0, SQUARE * 4 + COLOR_WIDTH * 4)]
    }

    pub fn transparent_color(&self) -> u8 {
        self.sprite[(0, SQUARE * 4 + COLOR_WIDTH * 5)]
    }

    pub fn glyph_text_color(&self) -> u8 {
        self.extended_color_idx
    }

    pub fn move_color(&self, glyph: MoveGlyph) -> u8 {
        self.extended_color_idx + glyph.index()
    }

    pub fn square(&self) -> usize {
        SQUARE
    }

    pub fn width(&self) -> usize {
        self.square() * 8
    }

    pub fn square_dark_color(&self) -> u8 {
        self.sprite[(0, SQUARE)]
    }

    pub fn square_light_color(&self) -> u8 {
        self.sprite[(0, 0)]
    }

    pub fn square_highlighted_dark_color(&self) -> u8 {
        self.sprite[(0, SQUARE * 3)]
    }

    pub fn square_highlighted_light_color(&self) -> u8 {
        self.sprite[(0, SQUARE * 2)]
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

    pub fn sprite(&self, key: &SpriteKey) -> ArrayView2<'_, u8> {
        let y = key.y();
        let x = key.x();
        self.sprite.slice(s!(
            (SQUARE * y)..(SQUARE + SQUARE * y),
            (SQUARE * x)..(SQUARE + SQUARE * x)
        ))
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
