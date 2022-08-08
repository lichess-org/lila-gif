use std::{iter::FusedIterator, vec};

use bytes::{BufMut, Bytes, BytesMut};
use gift::{block, Encoder};
use ndarray::{s, ArrayViewMut2};
use rusttype::Scale;
use shakmaty::{uci::Uci, Bitboard, Board};

use crate::{
    api::{Comment, Orientation, PieceName, PlayerName, RequestBody, RequestParams, ThemeName},
    theme::{SpriteKey, Theme, ThemeMap, DEFAULT_PIECE_NAME, DEFAULT_THEME_NAME},
};

enum RenderState {
    Preamble,
    Frame(RenderFrame),
    Complete,
}

struct PlayerBars {
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

#[derive(Default)]
struct RenderFrame {
    board: Board,
    highlighted: Bitboard,
    checked: Bitboard,
    delay: Option<u16>,
}

impl RenderFrame {
    fn diff(&self, prev: &RenderFrame) -> Bitboard {
        (prev.checked ^ self.checked)
            | (prev.highlighted ^ self.highlighted)
            | (prev.board.white() ^ self.board.white())
            | (prev.board.pawns() ^ self.board.pawns())
            | (prev.board.knights() ^ self.board.knights())
            | (prev.board.bishops() ^ self.board.bishops())
            | (prev.board.rooks() ^ self.board.rooks())
            | (prev.board.queens() ^ self.board.queens())
            | (prev.board.kings() ^ self.board.kings())
    }
}

fn get_theme_from_params(
    theme_map: &'static ThemeMap,
    theme_param: &Option<ThemeName>,
    piece_param: &Option<PieceName>,
) -> &'static Theme {
    let theme_name = match theme_param.is_some()
        && theme_map
            .map
            .contains_key(&theme_param.unwrap().to_string())
    {
        true => theme_param.unwrap().to_string(),
        false => DEFAULT_THEME_NAME.to_string(),
    };

    let piece_name = match piece_param.is_some()
        && theme_map.map[&DEFAULT_THEME_NAME.to_string()]
            .contains_key(&piece_param.unwrap().to_string())
    {
        true => piece_param.unwrap().to_string(),
        false => DEFAULT_PIECE_NAME.to_string(),
    };
    &theme_map.map[&theme_name][&piece_name]
}

pub struct Render {
    theme: &'static Theme,
    state: RenderState,
    buffer: Vec<u8>,
    comment: Option<Comment>,
    bars: Option<PlayerBars>,
    orientation: Orientation,
    frames: vec::IntoIter<RenderFrame>,
    kork: bool,
}

impl Render {
    pub fn new_image(theme_map: &'static ThemeMap, params: RequestParams) -> Render {
        let bars = params.white.is_some() || params.black.is_some();
        let theme = get_theme_from_params(theme_map, &params.theme, &params.piece);
        Render {
            theme,
            buffer: vec![0; theme.height(bars) * theme.width()],
            state: RenderState::Preamble,
            comment: params.comment,
            bars: PlayerBars::from(params.white, params.black),
            orientation: params.orientation,
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

    pub fn new_animation(theme_map: &'static ThemeMap, params: RequestBody) -> Render {
        let bars = params.white.is_some() || params.black.is_some();
        let default_delay = params.delay;
        let theme = get_theme_from_params(theme_map, &params.theme, &params.piece);
        Render {
            theme,
            buffer: vec![0; theme.height(bars) * theme.width()],
            state: RenderState::Preamble,
            comment: params.comment,
            bars: PlayerBars::from(params.white, params.black),
            orientation: params.orientation,
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

impl Iterator for Render {
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
                        self.orientation.fold(&bars.black, &bars.white),
                    );

                    render_bar(
                        view.slice_mut(s!((self.theme.bar_height() + self.theme.width()).., ..)),
                        self.theme,
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
                    None,
                    &frame,
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
                        Some(prev),
                        &frame,
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

impl FusedIterator for Render {}

fn render_diff(
    buffer: &mut [u8],
    theme: &Theme,
    orientation: Orientation,
    prev: Option<&RenderFrame>,
    frame: &RenderFrame,
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

        view.slice_mut(s!(
            top..(top + theme.square()),
            left..(left + theme.square())
        ))
        .assign(&theme.sprite(key));
    }

    (
        (theme.square() * x_min, theme.square() * y_min),
        (width, height),
    )
}

fn render_bar(mut view: ArrayViewMut2<u8>, theme: &Theme, player_name: &str) {
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

    let v_metrics = theme.font().v_metrics(scale);
    let glyphs = theme.font().layout(
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

fn highlight_uci(uci: Option<Uci>) -> Bitboard {
    match uci {
        Some(Uci::Normal { from, to, .. }) => Bitboard::from(from) | Bitboard::from(to),
        Some(Uci::Put { to, .. }) => Bitboard::from(to),
        _ => Bitboard::EMPTY,
    }
}
