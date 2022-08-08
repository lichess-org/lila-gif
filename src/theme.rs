use std::fs::DirEntry;
use std::io::Read;
use std::{collections::HashMap, fs};

use gift::block::{ColorTableConfig, GlobalColorTable};
use ndarray::{s, Array2, ArrayView2};
use rusttype::Font;
use shakmaty::{Piece, Role};

const SQUARE: usize = 90;
const COLOR_WIDTH: usize = 90 * 2 / 3;

pub const DEFAULT_THEME_NAME: &str = "brown";
pub const DEFAULT_PIECE_NAME: &str = "cburnett";

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
    color_table_config: ColorTableConfig,
    global_color_table: GlobalColorTable,
    sprite: Array2<u8>,
    font: Font<'static>,
}

impl Theme {
    pub fn new(file_path: &str) -> Theme {
        let mut f = std::fs::File::open(file_path).unwrap();
        let mut sprite_data = Vec::new();
        f.read_to_end(&mut sprite_data).unwrap();
        let mut decoder = gift::Decoder::new(std::io::Cursor::new(sprite_data)).into_frames();
        let preamble = decoder
            .preamble()
            .expect("decode preamble")
            .expect("preamble");
        let frame = decoder.next().expect("frame").expect("decode frame");
        let sprite =
            Array2::from_shape_vec((SQUARE * 8, SQUARE * 8), frame.image_data.data().to_owned())
                .expect("from shape");

        let font_data = include_bytes!("../theme/NotoSans-Regular.ttf") as &[u8];
        let font = Font::try_from_bytes(font_data).expect("parse font");

        Theme {
            color_table_config: preamble.logical_screen_desc.color_table_config(),
            global_color_table: preamble.global_color_table.expect("color table present"),
            sprite,
            font,
        }
    }

    pub fn font(&self) -> &Font {
        &self.font
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

pub struct ThemeMap {
    // Map of theme_name to map of piece_name to Theme
    // map["theme name"]["piece name"] -> Theme
    pub map: HashMap<String, HashMap<String, Theme>>,
}

impl ThemeMap {
    pub fn new() -> ThemeMap {
        ThemeMap {
            map: HashMap::new(),
        }
    }

    pub fn initialize(mut self) -> ThemeMap {
        let paths = fs::read_dir("./theme/sprites/").unwrap();
        for maybe_path in paths {
            if let Result::Ok(path) = maybe_path {
                let _ = &mut self.insert_theme_from_path(path);
            }
        }
        println!("initialized theme map");
        self
    }

    fn insert_theme_from_path(&mut self, path: DirEntry) -> Option<()> {
        let file_name_string = path.file_name().as_os_str().to_string_lossy().to_string();
        let file_name = file_name_string.split(".").next()?;
        let mut file_name_parts = file_name.split("-");
        let board_theme = file_name_parts.next()?.to_string();
        let piece_set = file_name_parts.next()?.to_string();
        let sub_map = self
            .map
            .entry(board_theme.clone())
            .or_insert(HashMap::new());
        sub_map.insert(piece_set.clone(), Theme::new(path.path().to_str().unwrap()));
        Some(())
    }
}
