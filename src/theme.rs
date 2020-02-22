use gift::block::Preamble;
use rusttype::FontCollection;
use rusttype::Font;

pub struct Theme {
    preamble: Preamble,
    font: Font<'static>,
}

impl Theme {
    pub fn new() -> Theme {
        println!("hello world");
        let theme_data = include_bytes!("../theme/theme.gif") as &[u8];
        let mut decoder = gift::Decoder::new(std::io::Cursor::new(theme_data)).into_frame_decoder();
        let preamble = decoder.preamble().expect("decode preamble").expect("preamble");

        println!("preamble: {:?}", preamble);
        let frame = decoder.next().expect("frame").expect("decode frame");
        dbg!(frame.graphic_control_ext);
        dbg!(frame.image_data.data().len());

        let font_data = include_bytes!("../theme/NotoSans-Regular.ttf") as &[u8];
        let font = FontCollection::from_bytes(font_data)
            .expect("font collection")
            .into_font()
            .expect("single font");

        Theme {
            preamble,
            font,
        }
    }
}
