use rusttype::FontCollection;
use rusttype::Font;

pub struct Theme {
    font: Font<'static>,
}

impl Theme {
    pub fn new() -> Theme {
        let font_data = include_bytes!("../theme/NotoSans-Regular.ttf");

        let font = FontCollection::from_bytes(font_data as &[u8])
            .expect("font collection")
            .into_font()
            .expect("single font");

        Theme {
            font
        }
    }
}
