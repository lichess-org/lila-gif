use gift::block::Preamble;
use rusttype::FontCollection;
use rusttype::Font;
use ndarray::Array2;

pub struct Theme {
    preamble: Preamble,
    sprites: Array2<u8>,
    font: Font<'static>,
}

impl Theme {
    pub fn new() -> Theme {
        let theme_data = include_bytes!("../theme/theme.gif") as &[u8];
        let mut decoder = gift::Decoder::new(std::io::Cursor::new(theme_data)).into_frames();
        let preamble = decoder.preamble().expect("decode preamble").expect("preamble");
        let frame = decoder.next().expect("frame").expect("decode frame");
        let sprites = Array2::from_shape_vec((720, 720), frame.image_data.data().to_owned()).expect("from shape");

        dbg!(frame.graphic_control_ext);
        dbg!(frame.image_data.data().len());

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

    pub fn background_color(&self) -> u8 {
        self.sprites[(0, 0)]
    }
}
