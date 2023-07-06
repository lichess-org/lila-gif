use std::vec;

use bytes::{BufMut, Bytes, BytesMut};
use gift::{block, Encoder};
use ndarray::{s, ArrayViewMut2};
use rusttype::{Font, LayoutIter, Scale};
use shakmaty::{Bitboard, File, Rank, Square};

use super::renderer::{highlight_uci, RenderFrame, RenderState, SpriteKey};
use crate::{
    api::{Comment, Coordinates, Orientation, PlayerName, RequestBody, RequestParams},
    theme::{Theme, Themes},
};

pub struct GIFRenderer {
    theme: &'static Theme,
    font: &'static Font<'static>,
    state: RenderState,
    buffer: Vec<u8>,
    comment: Option<Comment>,
    bars: Option<PlayerBars>,
    orientation: Orientation,
    coordinates: Coordinates,
    frames: vec::IntoIter<RenderFrame>,
    kork: bool,
}

impl GIFRenderer {
    pub fn new_image(themes: &'static Themes, params: RequestParams) -> GIFRenderer {
        let bars = PlayerBars::from(params.white, params.black);
        let theme = themes.get(params.theme, params.piece);
        let buffer = vec![0; theme.height(bars.is_some()) * theme.width()];
        GIFRenderer {
            theme,
            font: themes.font(),
            buffer,
            state: RenderState::Preamble,
            comment: params.comment,
            bars,
            orientation: params.orientation,
            coordinates: params.coordinates,
            frames: vec![RenderFrame {
                highlighted: highlight_uci(params.last_move),
                checked: params.check.to_square(&params.fen.0).into_iter().collect(),
                board: params.fen.0.board,
                delay: None,
            }]
            .into_iter(),
            kork: false,
        }
    }
    pub fn new_animation(themes: &'static Themes, params: RequestBody) -> GIFRenderer {
        let bars = PlayerBars::from(params.white, params.black);
        let default_delay = params.delay;
        let theme = themes.get(params.theme, params.piece);
        GIFRenderer {
            theme,
            font: themes.font(),
            buffer: vec![0; theme.height(bars.is_some()) * theme.width()],
            state: RenderState::Preamble,
            comment: params.comment,
            bars,
            orientation: params.orientation,
            coordinates: params.coordinates,
            frames: params
                .frames
                .into_iter()
                .map(|frame| RenderFrame {
                    highlighted: highlight_uci(frame.last_move),
                    checked: frame.check.to_square(&frame.fen.0).into_iter().collect(),
                    board: frame.fen.0.board,
                    delay: Some(frame.delay.unwrap_or(default_delay)),
                })
                .collect::<Vec<_>>()
                .into_iter(),
            kork: true,
        }
    }
}

impl Iterator for GIFRenderer {
    type Item = Bytes;

    fn next(&mut self) -> Option<Bytes> {
        let mut output = BytesMut::new().writer();
        match self.state {
            RenderState::Preamble => {
                let mut blocks = Encoder::new(&mut output).into_block_enc();

                blocks.encode(block::Header::default()).expect("enc header");

                blocks
                    .encode(
                        block::LogicalScreenDesc::default()
                            .with_screen_height(self.theme.height(self.bars.is_some()) as u16)
                            .with_screen_width(self.theme.width() as u16)
                            .with_color_table_config(self.theme.color_table_config()),
                    )
                    .expect("enc logical screen desc");

                blocks
                    .encode(self.theme.global_color_table().clone())
                    .expect("enc global color table");

                blocks
                    .encode(block::Application::with_loop_count(0))
                    .expect("enc application");

                let comment = self
                    .comment
                    .as_ref()
                    .map_or("https://github.com/lichess-org/lila-gif".as_bytes(), |c| {
                        c.as_bytes()
                    });
                if !comment.is_empty() {
                    let mut comments = block::Comment::default();
                    comments.add_comment(comment);
                    blocks.encode(comments).expect("enc comment");
                }

                let mut view = ArrayViewMut2::from_shape(
                    (self.theme.height(self.bars.is_some()), self.theme.width()),
                    &mut self.buffer,
                )
                .expect("shape");

                let mut board_view = if let Some(ref bars) = self.bars {
                    render_bar(
                        view.slice_mut(s!(..self.theme.bar_height(), ..)),
                        self.theme,
                        self.font,
                        self.orientation.fold(&bars.black, &bars.white),
                    );

                    render_bar(
                        view.slice_mut(s!((self.theme.bar_height() + self.theme.width()).., ..)),
                        self.theme,
                        self.font,
                        self.orientation.fold(&bars.white, &bars.black),
                    );

                    view.slice_mut(s!(
                        self.theme.bar_height()..(self.theme.bar_height() + self.theme.width()),
                        ..
                    ))
                } else {
                    view
                };

                let frame = self.frames.next().unwrap_or_default();

                if let Some(delay) = frame.delay {
                    let mut ctrl = block::GraphicControl::default();
                    ctrl.set_delay_time_cs(delay);
                    blocks.encode(ctrl).expect("enc graphic control");
                }

                render_diff(
                    board_view.as_slice_mut().expect("continguous"),
                    self.theme,
                    self.orientation,
                    self.coordinates,
                    None,
                    &frame,
                    self.font,
                );

                blocks
                    .encode(
                        block::ImageDesc::default()
                            .with_height(self.theme.height(self.bars.is_some()) as u16)
                            .with_width(self.theme.width() as u16),
                    )
                    .expect("enc image desc");

                let mut image_data = block::ImageData::new(self.buffer.len());
                image_data.data_mut().extend_from_slice(&self.buffer);
                blocks.encode(image_data).expect("enc image data");

                self.state = RenderState::Frame(frame);
            }
            RenderState::Frame(ref prev) => {
                let mut blocks = Encoder::new(&mut output).into_block_enc();

                if let Some(frame) = self.frames.next() {
                    let mut ctrl = block::GraphicControl::default();
                    ctrl.set_disposal_method(block::DisposalMethod::Keep);
                    ctrl.set_transparent_color_idx(self.theme.transparent_color());
                    if let Some(delay) = frame.delay {
                        ctrl.set_delay_time_cs(delay);
                    }
                    blocks.encode(ctrl).expect("enc graphic control");

                    let ((left, y), (w, h)) = render_diff(
                        &mut self.buffer,
                        self.theme,
                        self.orientation,
                        self.coordinates,
                        Some(prev),
                        &frame,
                        self.font,
                    );

                    let top = y + if self.bars.is_some() {
                        self.theme.bar_height()
                    } else {
                        0
                    };

                    blocks
                        .encode(
                            block::ImageDesc::default()
                                .with_left(left as u16)
                                .with_top(top as u16)
                                .with_height(h as u16)
                                .with_width(w as u16),
                        )
                        .expect("enc image desc");

                    let mut image_data = block::ImageData::new(w * h);
                    image_data
                        .data_mut()
                        .extend_from_slice(&self.buffer[..(w * h)]);
                    blocks.encode(image_data).expect("enc image data");

                    self.state = RenderState::Frame(frame);
                } else {
                    // Add a black frame at the end, to work around twitter
                    // cutting off the last frame.
                    if self.kork {
                        let mut ctrl = block::GraphicControl::default();
                        ctrl.set_disposal_method(block::DisposalMethod::Keep);
                        ctrl.set_transparent_color_idx(self.theme.transparent_color());
                        ctrl.set_delay_time_cs(1);
                        blocks.encode(ctrl).expect("enc graphic control");

                        let height = self.theme.height(self.bars.is_some());
                        let width = self.theme.width();
                        blocks
                            .encode(
                                block::ImageDesc::default()
                                    .with_left(0)
                                    .with_top(0)
                                    .with_height(height as u16)
                                    .with_width(width as u16),
                            )
                            .expect("enc image desc");

                        let mut image_data = block::ImageData::new(height * width);
                        image_data
                            .data_mut()
                            .resize(height * width, self.theme.bar_color());
                        blocks.encode(image_data).expect("enc image data");
                    }

                    blocks
                        .encode(block::Trailer::default())
                        .expect("enc trailer");
                    self.state = RenderState::Complete;
                }
            }
            RenderState::Complete => return None,
        }
        Some(output.into_inner().freeze())
    }
}

pub struct PlayerBars {
    white: PlayerName,
    black: PlayerName,
}

impl PlayerBars {
    fn from(white: Option<PlayerName>, black: Option<PlayerName>) -> Option<PlayerBars> {
        if white.is_some() || black.is_some() {
            Some(PlayerBars {
                white: white.unwrap_or_default(),
                black: black.unwrap_or_default(),
            })
        } else {
            None
        }
    }
}

fn render_diff(
    buffer: &mut [u8],
    theme: &Theme,
    orientation: Orientation,
    coordinates: Coordinates,
    prev: Option<&RenderFrame>,
    frame: &RenderFrame,
    font: &Font,
) -> ((usize, usize), (usize, usize)) {
    let diff = prev.map_or(Bitboard::FULL, |p| p.diff(frame));

    let x_min = diff
        .into_iter()
        .map(|sq| orientation.x(sq))
        .min()
        .unwrap_or(0);
    let y_min = diff
        .into_iter()
        .map(|sq| orientation.y(sq))
        .min()
        .unwrap_or(0);
    let x_max = diff
        .into_iter()
        .map(|sq| orientation.x(sq))
        .max()
        .unwrap_or(0)
        + 1;
    let y_max = diff
        .into_iter()
        .map(|sq| orientation.y(sq))
        .max()
        .unwrap_or(0)
        + 1;

    let width = (x_max - x_min) * theme.square();
    let height = (y_max - y_min) * theme.square();

    let mut view = ArrayViewMut2::from_shape((height, width), buffer).expect("shape");

    if prev.is_some() {
        view.fill(theme.transparent_color());
    }

    for sq in diff {
        let key = SpriteKey {
            piece: frame.board.piece_at(sq),
            dark_square: sq.is_dark(),
            highlight: frame.highlighted.contains(sq),
            check: frame.checked.contains(sq),
        };

        let left = (orientation.x(sq) - x_min) * theme.square();
        let top = (orientation.y(sq) - y_min) * theme.square();

        let mut square_buffer = view.slice_mut(s!(
            top..(top + theme.square()),
            left..(left + theme.square())
        ));

        square_buffer.assign(&theme.sprite(&key));

        if coordinates == Coordinates::Yes {
            let coords_scale: Scale = Scale { x: 30.0, y: 30.0 };
            let (coords_rank, coords_file) = match orientation {
                Orientation::White => (Rank::First, File::H),
                Orientation::Black => (Rank::Eighth, File::A),
            };
            if sq.rank() == coords_rank {
                render_file(&mut square_buffer, &sq, &key, theme, font, coords_scale)
            };
            if sq.file() == coords_file {
                render_rank(&mut square_buffer, &sq, &key, theme, font, coords_scale)
            };
        }
    }

    (
        (theme.square() * x_min, theme.square() * y_min),
        (width, height),
    )
}

fn render_file(
    square_buffer: &mut ArrayViewMut2<u8>,
    sq: &Square,
    sprite_key: &SpriteKey,
    theme: &Theme,
    font: &Font,
    font_scale: Scale,
) {
    let v_metrics = font.v_metrics(font_scale);
    let square_file = format!("{}", sq.file());
    let glyphs = font.layout(
        &square_file,
        font_scale,
        rusttype::point(5.0, theme.square() as f32 + v_metrics.descent),
    );
    let text_color = get_square_background_color(sprite_key.highlight, sq.is_light(), theme);
    let background_color = get_square_background_color(sprite_key.highlight, sq.is_dark(), theme);

    render_coord(square_buffer, glyphs, theme, text_color, background_color)
}

fn render_rank(
    square_buffer: &mut ArrayViewMut2<u8>,
    sq: &Square,
    sprite_key: &SpriteKey,
    theme: &Theme,
    font: &Font,
    font_scale: Scale,
) {
    let v_metrics = font.v_metrics(font_scale);
    let square_rank = format!("{}", sq.rank());
    let glyphs = font.layout(
        &square_rank,
        font_scale,
        rusttype::point(theme.square() as f32 - 15.0, v_metrics.ascent),
    );
    let text_color = get_square_background_color(sprite_key.highlight, sq.is_light(), theme);
    let background_color = get_square_background_color(sprite_key.highlight, sq.is_dark(), theme);

    render_coord(square_buffer, glyphs, theme, text_color, background_color)
}

fn render_bar(mut view: ArrayViewMut2<u8>, theme: &Theme, font: &Font, player_name: &str) {
    view.fill(theme.bar_color());

    let mut text_color = theme.text_color();
    if player_name.starts_with("BOT ") {
        text_color = theme.bot_color();
    } else {
        for title in &[
            "GM ", "WGM ", "IM ", "WIM ", "FM ", "WFM ", "NM ", "CM ", "WCM ", "WNM ", "LM ",
            "BOT ",
        ] {
            if player_name.starts_with(title) {
                text_color = theme.gold_color();
                break;
            }
        }
    }

    let height = 40.0;
    let padding = 10.0;
    let scale = Scale {
        x: height,
        y: height,
    };

    let v_metrics = font.v_metrics(scale);
    let glyphs = font.layout(
        player_name,
        scale,
        rusttype::point(padding, padding + v_metrics.ascent),
    );

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            // Poor man's anti-aliasing.
            g.draw(|left, top, intensity| {
                let left = left as i32 + bb.min.x;
                let top = top as i32 + bb.min.y;
                if 0 <= left
                    && left < theme.width() as i32
                    && 0 <= top
                    && top < theme.bar_height() as i32
                    && intensity >= 0.01
                {
                    if intensity < 0.5 && text_color == theme.text_color() {
                        view[(top as usize, left as usize)] = theme.med_text_color();
                    } else {
                        view[(top as usize, left as usize)] = text_color;
                    }
                }
            });
        } else {
            text_color = theme.text_color();
        }
    }
}

fn render_coord(
    square_buffer: &mut ArrayViewMut2<u8>,
    glyphs: LayoutIter,
    theme: &Theme,
    text_color: u8,
    background_color: u8,
) {
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            // Poor man's anti-aliasing.
            g.draw(|left, top, intensity| {
                let left = left as i32 + bb.min.x;
                let top = top as i32 + bb.min.y;
                if 0 <= left && left < theme.width() as i32 && 0 <= top && intensity >= 0.01 {
                    if intensity < 0.5 {
                        square_buffer[(top as usize, left as usize)] = background_color;
                    } else {
                        square_buffer[(top as usize, left as usize)] = text_color;
                    }
                }
            });
        };
    }
}

fn get_square_background_color(is_highlighted: bool, is_dark: bool, theme: &Theme) -> u8 {
    match is_highlighted {
        true => match is_dark {
            true => theme.square_highlighted_dark_color(),
            false => theme.square_highlighted_light_color(),
        },
        false => match is_dark {
            true => theme.square_dark_color(),
            false => theme.square_light_color(),
        },
    }
}
