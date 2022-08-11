use gif::Decoder;
use ndarray::{s, Array2, ArrayView2};
use rusttype::Font;
use shakmaty::{Piece, Role};

use crate::assets::{sprite_data, BoardTheme, ByBoardTheme, ByPieceSet, PieceSet};

const SQUARE: usize = 90;
const COLOR_WIDTH: usize = 90 * 2 / 3;

pub struct SpriteKey {
    pub piece: Option<Piece>,
    pub dark_square: bool,
    pub highlight: bool,
    pub check: bool,
}

impl SpriteKey {
    fn x(&self) -> usize {
        (if self.piece.map_or(false, |p| p.color.is_white()) {
            4
        } else {
            0
        }) + (if self.highlight { 2 } else { 0 })
            + (if self.dark_square { 1 } else { 0 })
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
    global_color_table: Vec<u8>,
    sprite: Array2<u8>,
}

impl Theme {
    fn new(sprite_data: &[u8]) -> Theme {
        let mut decoder = Decoder::new(std::io::Cursor::new(sprite_data)).unwrap();
        let frame = decoder.read_next_frame().unwrap().expect("no frames");
        let sprite = Array2::from_shape_vec((SQUARE * 8, SQUARE * 8), frame.buffer.to_vec())
            .expect("from shape");

        Theme {
            global_color_table: decoder.global_palette().unwrap().to_vec(),
            sprite,
        }
    }

    pub fn global_color_table(&self) -> &Vec<u8> {
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

    pub fn sprite(&self, key: SpriteKey) -> ArrayView2<u8> {
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
        let font_data = include_bytes!("../theme/NotoSans-Regular.ttf") as &[u8];
        let font = Font::try_from_bytes(font_data).expect("parse font");

        Themes {
            map: ByBoardTheme::new(|board| {
                ByPieceSet::new(|pieces| Theme::new(sprite_data(board, pieces)))
            }),
            font,
        }
    }

    pub fn font(&self) -> &Font {
        &self.font
    }

    pub fn get(&self, board: BoardTheme, pieces: PieceSet) -> &Theme {
        self.map.by_board_theme(board).by_piece_set(pieces)
    }
}
