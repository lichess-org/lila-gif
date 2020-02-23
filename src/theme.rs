use gift::block::Preamble;
use rusttype::{Font, FontCollection, Scale};
use ndarray::{Array2, ArrayView2, ArrayViewMut2, s};
use shakmaty::{Piece, Role};

const SQUARE: usize = 90;

pub struct SpriteKey {
    pub piece: Option<Piece>,
    pub dark_square: bool,
    pub last_move: bool,
    pub check: bool,
}

impl SpriteKey {
    fn x(&self) -> usize {
        let a = (if self.piece.map_or(false, |p| p.color.is_white()) { 4 } else { 0 });
        let b = (if self.last_move { 2 } else { 0 });
        let c = (if self.dark_square { 1 } else { 0 });
        a + b + c
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
    pub(crate) preamble: Preamble,
    sprites: Array2<u8>,
    font: Font<'static>,
}

impl Theme {
    pub fn new() -> Theme {
        let theme_data = include_bytes!("../theme/theme.gif") as &[u8];
        let mut decoder = gift::Decoder::new(std::io::Cursor::new(theme_data)).into_frames();
        let preamble = decoder.preamble().expect("decode preamble").expect("preamble");
        let frame = decoder.next().expect("frame").expect("decode frame");
        let sprites = Array2::from_shape_vec((SQUARE * 8, SQUARE * 8), frame.image_data.data().to_owned()).expect("from shape");

        let font_data = include_bytes!("../theme/NotoSans-Regular.ttf") as &[u8];
        let font = FontCollection::from_bytes(font_data)
            .expect("font collection")
            .into_font()
            .expect("single font");

        Theme {
            preamble,
            sprites,
            font,
        }
    }

    pub fn light_color(&self) -> u8 {
        self.sprites[(0, 0)]
    }

    pub fn dark_color(&self) -> u8 {
        self.sprites[(0, SQUARE)]
    }

    pub fn highlighted_light_color(&self) -> u8 {
        self.sprites[(0, SQUARE * 2)]
    }

    pub fn highlighted_dark_color(&self) -> u8 {
        self.sprites[(0, SQUARE * 3)]
    }

    pub fn bar_color(&self) -> u8 {
        self.sprites[(0, SQUARE * 4)]
    }

    pub fn text_color(&self) -> u8 {
        self.sprites[(0, SQUARE * 5)]
    }

    pub fn gold_color(&self) -> u8 {
        self.sprites[(0, SQUARE * 6)]
    }

    pub fn transparent_color(&self) -> u8 {
        self.sprites[(0, SQUARE * 7)]
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

    pub fn height(&self) -> usize {
        self.width() + 2 * self.bar_height()
    }

    pub fn sprite(&self, key: SpriteKey) -> ArrayView2<u8> {
        let y = key.y();
        let x = key.x();
        self.sprites.slice(s!((SQUARE * y)..(SQUARE + SQUARE * y), (SQUARE * x)..(SQUARE + SQUARE * x)))
    }

    pub fn render_bar(&self, mut view: ArrayViewMut2<u8>, player_name: &str) {
        view.fill(self.bar_color());

        let mut text_color = if player_name.contains(' ') {
            self.gold_color() // title
        } else {
            self.text_color()
        };
        let gold_color = self.gold_color();
        let padding = 10;
        let height = 40.0;
        let scale = Scale {
            x: height,
            y: height,
        };
        let v_metrics = self.font.v_metrics(scale);
        let glyphs = self.font.layout(player_name, scale, rusttype::point(10.0, 10.0 + v_metrics.ascent));
        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, intensity| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    if intensity > 0.01 && 0 <= x && x < self.width() as i32 && 0 <= y && y < self.bar_height() as i32 {
                        view[(y as usize, x as usize)] = text_color;
                    }
                });
            } else {
                text_color = self.text_color();
            }
        }
    }
}
