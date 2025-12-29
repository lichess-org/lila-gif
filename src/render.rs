use std::{iter::FusedIterator, vec};

use bytes::{BufMut, Bytes, BytesMut};
use gift::{block, Encoder};
use ndarray::{s, ArrayView2, ArrayViewMut2};
use rusttype::{Font, LayoutIter, Scale};
use shakmaty::{uci::UciMove, Bitboard, Board, File, Rank, Square};

use crate::{
    api::{Comment, Coordinates, MoveGlyph, Orientation, PlayerName, RequestBody, RequestParams},
    theme::{SpriteKey, Theme, Themes},
};

const GLYPH_BADGE_RADIUS: f32 = 18.0;
const CLOCK_REGION_PADDING: usize = 20;

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
    fn from(
        white: Option<PlayerName>,
        black: Option<PlayerName>,
        has_clocks: bool,
    ) -> Option<PlayerBars> {
        let white_name = white.filter(|s| !s.is_empty());
        let black_name = black.filter(|s| !s.is_empty());

        if white_name.is_some() || black_name.is_some() || has_clocks {
            Some(PlayerBars {
                white: white_name.unwrap_or_default(),
                black: black_name.unwrap_or_default(),
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
    glyph: Option<MoveGlyph>,
    white_clock: Option<u32>,
    black_clock: Option<u32>,
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

pub struct Render {
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
    clock_widths: [usize; 2],
}

impl Render {
    pub fn new_image(themes: &'static Themes, params: RequestParams) -> Render {
        let bars = PlayerBars::from(params.white, params.black, false);
        let theme = themes.get(params.theme, params.piece);
        Render {
            theme,
            font: themes.font(),
            buffer: vec![0; theme.height(bars.is_some()) * theme.width()],
            state: RenderState::Preamble,
            comment: params.comment,
            bars,
            orientation: params.orientation,
            coordinates: params.coordinates,
            frames: vec![RenderFrame {
                highlighted: highlight_uci(params.last_move),
                checked: params
                    .check
                    .to_square(params.fen.as_setup())
                    .into_iter()
                    .collect(),
                board: params.fen.into_setup().board,
                delay: None,
                glyph: None,
                white_clock: None,
                black_clock: None,
            }]
            .into_iter(),
            kork: false,
            clock_widths: [0; 2],
        }
    }

    pub fn new_animation(themes: &'static Themes, params: RequestBody) -> Render {
        let clocks = params.clocks;
        let bars = PlayerBars::from(params.white, params.black, clocks.is_some());
        let default_delay = params.delay;
        let theme = themes.get(params.theme, params.piece);
        Render {
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
                .enumerate()
                .map(|(i, frame)| {
                    let white_clock = clocks
                        .as_ref()
                        .and_then(|c| c.white.get(i.saturating_sub(1) / 2).copied());
                    let black_clock = clocks
                        .as_ref()
                        .and_then(|c| c.black.get(i.saturating_sub(2) / 2).copied());
                    RenderFrame {
                        highlighted: highlight_uci(frame.last_move),
                        checked: frame
                            .check
                            .to_square(frame.fen.as_setup())
                            .into_iter()
                            .collect(),
                        board: frame.fen.into_setup().board,
                        delay: Some(frame.delay.unwrap_or(default_delay)),
                        glyph: frame.glyph,
                        white_clock,
                        black_clock,
                    }
                })
                .collect::<Vec<_>>()
                .into_iter(),
            kork: true,
            clock_widths: [0; 2],
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

                let frame = self.frames.next().unwrap_or_default();
                let mut view = ArrayViewMut2::from_shape(
                    (self.theme.height(self.bars.is_some()), self.theme.width()),
                    &mut self.buffer,
                )
                .expect("shape");

                let mut board_view = if let Some(ref bars) = self.bars {
                    let bar_height = self.theme.bar_height();
                    let btm_bar_y = bar_height + self.theme.width();
                    let bar_names = self.orientation.fold(
                        [(&bars.black as &str, 0), (&bars.white, btm_bar_y)],
                        [(&bars.white as &str, 0), (&bars.black, btm_bar_y)],
                    );
                    for (name, bar_top) in bar_names {
                        render_bar(
                            view.slice_mut(s!(bar_top..(bar_top + bar_height), ..)),
                            self.theme,
                            self.font,
                            name,
                        );
                    }

                    let mut clock_buffer = vec![0u8; bar_height * self.theme.width()];
                    for (clock, bar_top, idx) in
                        clock_positions(&frame, self.orientation, btm_bar_y)
                    {
                        if let Some(centis) = clock {
                            let region_width = render_clock_region(
                                &mut clock_buffer,
                                self.theme,
                                self.font,
                                centis,
                                self.clock_widths[idx],
                            );
                            self.clock_widths[idx] = region_width;
                            let clock_left =
                                self.theme.width() - region_width - CLOCK_REGION_PADDING;
                            let src = ArrayView2::from_shape(
                                (bar_height, region_width),
                                &clock_buffer[..bar_height * region_width],
                            )
                            .expect("clock src");
                            view.slice_mut(s!(
                                bar_top..(bar_top + bar_height),
                                clock_left..(clock_left + region_width)
                            ))
                            .assign(&src);
                        }
                    }

                    view.slice_mut(s!(bar_height..(bar_height + self.theme.width()), ..))
                } else {
                    view
                };

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
                    if self.bars.is_some() {
                        let bar_height = self.theme.bar_height();
                        let btm_bar_y = bar_height + self.theme.width();
                        let prev_clocks = clock_positions(prev, self.orientation, btm_bar_y);
                        let curr_clocks = clock_positions(&frame, self.orientation, btm_bar_y);

                        for ((clock, bar_top, idx), (prev_clock, _, _)) in
                            curr_clocks.into_iter().zip(prev_clocks)
                        {
                            let Some(centis) = clock.filter(|&c| Some(c) != prev_clock) else {
                                continue;
                            };
                            let mut ctrl = block::GraphicControl::default();
                            ctrl.set_disposal_method(block::DisposalMethod::Keep);
                            blocks.encode(ctrl).expect("enc clock ctrl");

                            let region_width = render_clock_region(
                                &mut self.buffer,
                                self.theme,
                                self.font,
                                centis,
                                self.clock_widths[idx],
                            );
                            self.clock_widths[idx] = region_width;
                            let region_size = bar_height * region_width;
                            let clock_left =
                                self.theme.width() - region_width - CLOCK_REGION_PADDING;

                            blocks
                                .encode(
                                    block::ImageDesc::default()
                                        .with_left(clock_left as u16)
                                        .with_top(bar_top as u16)
                                        .with_height(bar_height as u16)
                                        .with_width(region_width as u16),
                                )
                                .expect("enc clock desc");

                            let mut image_data = block::ImageData::new(region_size);
                            image_data
                                .data_mut()
                                .extend_from_slice(&self.buffer[..region_size]);
                            blocks.encode(image_data).expect("enc clock data");
                        }
                    }

                    let mut ctrl = block::GraphicControl::default();
                    ctrl.set_disposal_method(block::DisposalMethod::Keep);
                    ctrl.set_transparent_color(Some(self.theme.transparent_color()));
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
                        ctrl.set_transparent_color(Some(self.theme.transparent_color()));
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

fn render_glyph_badge(
    square_buffer: &mut ArrayViewMut2<u8>,
    theme: &Theme,
    font: &Font,
    glyph: MoveGlyph,
) {
    let square_size = theme.square();
    let center_x = square_size as f32 - GLYPH_BADGE_RADIUS;
    let center_y = GLYPH_BADGE_RADIUS;
    let bg_color = theme.move_color(glyph);
    let inner_radius_sq = (GLYPH_BADGE_RADIUS - 0.5) * (GLYPH_BADGE_RADIUS - 0.5);
    let min_x = (center_x - GLYPH_BADGE_RADIUS).max(0.0) as usize;
    let max_x = ((center_x + GLYPH_BADGE_RADIUS).ceil() as usize).min(square_size);
    let min_y = (center_y - GLYPH_BADGE_RADIUS).max(0.0) as usize;
    let max_y = ((center_y + GLYPH_BADGE_RADIUS).ceil() as usize).min(square_size);

    for y in min_y..max_y {
        for x in min_x..max_x {
            let dx = x as f32 + 0.5 - center_x;
            let dy = y as f32 + 0.5 - center_y;
            if dx * dx + dy * dy <= inner_radius_sq {
                square_buffer[(y, x)] = bg_color;
            }
        }
    }

    let scale = Scale { x: 32.0, y: 32.0 };
    let glyphs: Vec<_> = font
        .layout(glyph.into(), scale, rusttype::point(0.0, 0.0))
        .collect();
    let (gmin_x, gmax_x, gmin_y, gmax_y) =
        glyphs.iter().filter_map(|g| g.pixel_bounding_box()).fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |(min_x, max_x, min_y, max_y), bb| {
                (
                    min_x.min(bb.min.x),
                    max_x.max(bb.max.x),
                    min_y.min(bb.min.y),
                    max_y.max(bb.max.y),
                )
            },
        );
    let offset_x = center_x - (gmax_x + gmin_x) as f32 / 2.0;
    let offset_y = center_y - (gmax_y + gmin_y) as f32 / 2.0;
    let text_color = theme.glyph_text_color();

    for g in &glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|left, top, intensity| {
                let px = (left as i32 + bb.min.x) as f32 + offset_x;
                let py = (top as i32 + bb.min.y) as f32 + offset_y;
                if px >= 0.0
                    && px < square_size as f32
                    && py >= 0.0
                    && py < square_size as f32
                    && intensity >= 0.05
                {
                    square_buffer[(py as usize, px as usize)] = text_color;
                }
            });
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

        if let Some(glyph) = frame.glyph {
            if frame.highlighted.contains(sq) && frame.board.piece_at(sq).is_some() {
                render_glyph_badge(&mut square_buffer, theme, font, glyph);
            }
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

fn format_clock(centis: u32) -> String {
    let total_secs = centis / 100;
    let tenths = (centis % 100) / 10;
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, mins, secs)
    } else if mins > 0 {
        format!("{}:{:02}", mins, secs)
    } else {
        format!("00:{:02}.{}", secs, tenths)
    }
}

fn render_clock_region(
    buffer: &mut [u8],
    theme: &Theme,
    font: &Font,
    centis: u32,
    min_width: usize,
) -> usize {
    let bar_height = theme.bar_height();
    let scale = Scale { x: 36.0, y: 36.0 };
    let v_metrics = font.v_metrics(scale);

    let clock_str = format_clock(centis);
    let glyphs: Vec<_> = font
        .layout(&clock_str, scale, rusttype::point(0.0, 0.0))
        .collect();
    let text_width = glyphs
        .iter()
        .filter_map(|g| g.pixel_bounding_box())
        .map(|bb| bb.max.x)
        .max()
        .unwrap_or(0) as usize;

    let region_width = text_width.max(min_width);
    let text_offset = (region_width - text_width) as i32;
    let mut view = ArrayViewMut2::from_shape(
        (bar_height, region_width),
        &mut buffer[..bar_height * region_width],
    )
    .expect("clock region shape");
    view.fill(theme.bar_color());

    let clock_color = theme.text_color();
    for g in &glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|left, top, intensity| {
                let left = left as i32 + bb.min.x + text_offset;
                let top = top as i32 + bb.min.y + (10.0 + v_metrics.ascent) as i32;
                if left >= 0
                    && left < region_width as i32
                    && top >= 0
                    && top < bar_height as i32
                    && intensity >= 0.01
                {
                    view[(top as usize, left as usize)] = if intensity < 0.5 {
                        theme.med_text_color()
                    } else {
                        clock_color
                    };
                }
            });
        }
    }

    region_width
}

fn highlight_uci(uci: Option<UciMove>) -> Bitboard {
    match uci {
        Some(UciMove::Normal { from, to, .. }) => Bitboard::from(from) | Bitboard::from(to),
        Some(UciMove::Put { to, .. }) => Bitboard::from(to),
        _ => Bitboard::EMPTY,
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

fn clock_positions(
    frame: &RenderFrame,
    orientation: Orientation,
    btm_bar_y: usize,
) -> [(Option<u32>, usize, usize); 2] {
    orientation.fold(
        [(frame.black_clock, 0, 0), (frame.white_clock, btm_bar_y, 1)],
        [(frame.white_clock, 0, 0), (frame.black_clock, btm_bar_y, 1)],
    )
}
