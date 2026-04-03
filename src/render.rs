use std::{iter::FusedIterator, vec};

use bytes::{BufMut, Bytes, BytesMut};
use gift::{block, Encoder};
use ndarray::{s, ArrayView2, ArrayViewMut2};
use rusttype::{Font, LayoutIter, Scale};
use shakmaty::{uci::UciMove, Bitboard, Board, File, Rank, Square};

use crate::{
    api::{Comment, Coordinates, MoveGlyph, Orientation, PlayerName, RequestBody, RequestParams, RequestPockets, PocketData},
    theme::{SpriteKey, Theme, Themes},
};

const GLYPH_BADGE_RADIUS: f32 = 18.0;
const GLYPH_FONT_SIZE: f32 = 32.0;
const BAR_PADDING: f32 = 10.0;
const CLOCK_FONT_SIZE: f32 = 36.0;
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

#[derive(Default, Clone, PartialEq)]
struct RenderFrame {
    board: Board,
    highlighted: Bitboard,
    checked: Bitboard,
    delay: Option<u16>,
    glyph: Option<MoveGlyph>,
    white_clock: Option<u32>,
    black_clock: Option<u32>,
    pockets: Option<RequestPockets>,
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
    has_pockets: bool,
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
        let has_pockets = params.pockets.is_some();
        Render {
            theme,
            font: themes.font(),
            buffer: vec![0; theme.height(bars.is_some(), has_pockets) * theme.width()],
            state: RenderState::Preamble,
            comment: params.comment,
            bars,
            has_pockets,
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
                pockets: params.pockets,
            }]
            .into_iter(),
            kork: false,
            clock_widths: [0; 2],
        }
    }

    pub fn new_animation(themes: &'static Themes, params: RequestBody) -> Render {
        let has_clocks = params
            .frames
            .iter()
            .any(|f| f.clock.white.is_some() || f.clock.black.is_some());
        let has_pockets = params.frames.iter().any(|f| f.pockets.is_some());
        let bars = PlayerBars::from(params.white, params.black, has_clocks);
        let default_delay = params.delay;
        let theme = themes.get(params.theme, params.piece);
        Render {
            theme,
            font: themes.font(),
            buffer: vec![0; theme.height(bars.is_some(), has_pockets) * theme.width()],
            state: RenderState::Preamble,
            comment: params.comment,
            bars,
            has_pockets,
            orientation: params.orientation,
            coordinates: params.coordinates,
            frames: params
                .frames
                .into_iter()
                .map(|frame| RenderFrame {
                    highlighted: highlight_uci(frame.last_move),
                    checked: frame
                        .check
                        .to_square(frame.fen.as_setup())
                        .into_iter()
                        .collect(),
                    board: frame.fen.into_setup().board,
                    delay: Some(frame.delay.unwrap_or(default_delay)),
                    glyph: frame.glyph,
                    white_clock: frame.clock.white,
                    black_clock: frame.clock.black,
                    pockets: frame.pockets,
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
                            .with_screen_height(self.theme.height(self.bars.is_some(), self.has_pockets) as u16)
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
                    (self.theme.height(self.bars.is_some(), self.has_pockets), self.theme.width()),
                    &mut self.buffer,
                )
                .expect("shape");

                let bar_height = self.theme.bar_height();
                let pocket_height = self.theme.pocket_height();
                let board_width = self.theme.width();

                let mut top_offset = 0;

                if let Some(ref bars) = self.bars {
                    let btm_bar_y = self.theme.height(true, self.has_pockets) - bar_height;
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

                    let mut clock_buffer = vec![0u8; bar_height * board_width];
                    for (idx, (clock, bar_top)) in
                        clock_positions(&frame, self.orientation, btm_bar_y)
                            .into_iter()
                            .enumerate()
                    {
                        if let Some(centis) = clock {
                            let (region_width, clock_left) = render_clock_region(
                                &mut clock_buffer,
                                self.theme,
                                self.font,
                                centis,
                                self.clock_widths[idx],
                            );
                            self.clock_widths[idx] = region_width;
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
                    top_offset += bar_height;
                }

                if self.has_pockets {
                    let btm_pocket_y = top_offset + pocket_height + board_width;
                    let pockets = frame.pockets.clone().unwrap_or_default();
                    let (top_p, btm_p) = self.orientation.fold((&pockets.black, &pockets.white), (&pockets.white, &pockets.black));

                    render_pockets(view.slice_mut(s!(top_offset..(top_offset + pocket_height), ..)), self.theme, self.font, top_p, self.orientation.fold(shakmaty::Color::Black, shakmaty::Color::White));
                    render_pockets(view.slice_mut(s!(btm_pocket_y..(btm_pocket_y + pocket_height), ..)), self.theme, self.font, btm_p, self.orientation.fold(shakmaty::Color::White, shakmaty::Color::Black));
                    top_offset += pocket_height;
                }

                if let Some(delay) = frame.delay {
                    let mut ctrl = block::GraphicControl::default();
                    ctrl.set_delay_time_cs(delay);
                    blocks.encode(ctrl).expect("enc graphic control");
                }

                let mut board_view = view.slice_mut(s!(top_offset..(top_offset + board_width), ..));
                render_diff(
                    board_view.as_slice_mut().expect("contiguous"),
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
                            .with_height(self.theme.height(self.bars.is_some(), self.has_pockets) as u16)
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
                    let bar_height = self.theme.bar_height();
                    let pocket_height = self.theme.pocket_height();
                    let board_width = self.theme.width();

                    if self.bars.is_some() {
                        let btm_bar_y = self.theme.height(true, self.has_pockets) - bar_height;
                        let prev_clocks = clock_positions(prev, self.orientation, btm_bar_y);
                        let curr_clocks = clock_positions(&frame, self.orientation, btm_bar_y);

                        for (idx, ((clock, bar_top), (prev_clock, _))) in
                            curr_clocks.into_iter().zip(prev_clocks).enumerate()
                        {
                            let Some(centis) = clock.filter(|&c| Some(c) != prev_clock) else {
                                continue;
                            };
                            let mut ctrl = block::GraphicControl::default();
                            ctrl.set_disposal_method(block::DisposalMethod::Keep);
                            blocks.encode(ctrl).expect("enc clock ctrl");

                            let (region_width, clock_left) = render_clock_region(
                                &mut self.buffer,
                                self.theme,
                                self.font,
                                centis,
                                self.clock_widths[idx],
                            );
                            self.clock_widths[idx] = region_width;
                            let region_size = bar_height * region_width;

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

                    if self.has_pockets && frame.pockets != prev.pockets {
                        let top_pocket_y = if self.bars.is_some() { bar_height } else { 0 };
                        let btm_pocket_y = top_pocket_y + pocket_height + board_width;
                        let curr_pockets = frame.pockets.clone().unwrap_or_default();
                        let prev_pockets = prev.pockets.clone().unwrap_or_default();

                        let (top_color, btm_color) = self.orientation.fold((shakmaty::Color::Black, shakmaty::Color::White), (shakmaty::Color::White, shakmaty::Color::Black));
                        let (top_curr, btm_curr) = self.orientation.fold((&curr_pockets.black, &curr_pockets.white), (&curr_pockets.white, &curr_pockets.black));
                        let (top_prev, btm_prev) = self.orientation.fold((&prev_pockets.black, &prev_pockets.white), (&prev_pockets.white, &prev_pockets.black));

                        let pocket_regions = [
                            (top_pocket_y, top_curr, top_prev, top_color),
                            (btm_pocket_y, btm_curr, btm_prev, btm_color),
                        ];

                        for (y, curr, prev_p, color) in pocket_regions {
                            if curr != prev_p {
                                let mut ctrl = block::GraphicControl::default();
                                ctrl.set_disposal_method(block::DisposalMethod::Keep);
                                blocks.encode(ctrl).expect("enc pocket ctrl");

                                let mut pocket_view = ArrayViewMut2::from_shape((pocket_height, board_width), &mut self.buffer).expect("pocket shape");
                                render_pockets(pocket_view.view_mut(), self.theme, self.font, curr, color);

                                blocks.encode(block::ImageDesc::default()
                                    .with_top(y as u16)
                                    .with_height(pocket_height as u16)
                                    .with_width(board_width as u16)
                                ).expect("enc pocket desc");

                                let mut image_data = block::ImageData::new(pocket_height * board_width);
                                image_data.data_mut().extend_from_slice(&self.buffer[..pocket_height * board_width]);
                                blocks.encode(image_data).expect("enc pocket data");
                            }
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

                    let mut top = y;
                    if self.bars.is_some() { top += bar_height; }
                    if self.has_pockets { top += pocket_height; }

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

                        let height = self.theme.height(self.bars.is_some(), self.has_pockets);
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
    let inner_radius_sq = (GLYPH_BADGE_RADIUS - 0.5).powi(2);
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

    let scale = Scale {
        x: GLYPH_FONT_SIZE,
        y: GLYPH_FONT_SIZE,
    };
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

    if gmin_x == i32::MAX {
        return;
    }

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

fn render_pockets(
    mut view: ArrayViewMut2<u8>,
    theme: &Theme,
    font: &Font,
    pockets: &PocketData,
    color: shakmaty::Color,
) {
    view.fill(theme.bar_color());
    let roles = [
        (shakmaty::Role::Pawn, pockets.pawn),
        (shakmaty::Role::Knight, pockets.knight),
        (shakmaty::Role::Bishop, pockets.bishop),
        (shakmaty::Role::Rook, pockets.rook),
        (shakmaty::Role::Queen, pockets.queen),
    ];

    let square_size = theme.square();
    let pocket_h = theme.pocket_height();

    let mut x_offset = (theme.width().saturating_sub(5 * square_size)) / 2;

    let red_bg = theme.move_color(MoveGlyph::Blunder);
    let white_text = theme.glyph_text_color();

    for (role, count) in roles {
        let key = SpriteKey {
            piece: if count > 0 {
                Some(shakmaty::Piece { color, role })
            } else {
                None
            },
            dark_square: false,
            highlight: false,
            check: false,
        };
        let sprite = theme.sprite(&key);

        // Draw sprite
        for y in 0..square_size {
            for x in 0..square_size {
                let ty = y;
                let tx = x + x_offset;
                if ty < pocket_h && tx < theme.width() {
                    view[(ty, tx)] = sprite[(y, x)];
                }
            }
        }

        // Draw badge and count
        if count > 1 {
            let count_str = count.to_string();
            let font_scale = Scale::uniform(32.0);
            let v_metrics = font.v_metrics(font_scale);

            let badge_size = 32;
            let badge_x = x_offset + square_size - badge_size;
            let badge_y = square_size - badge_size;

            for y in 0..badge_size {
                for x in 0..badge_size {
                    let ty = badge_y + y;
                    let tx = badge_x + x;
                    if ty < pocket_h && tx < theme.width() {
                        view[(ty, tx)] = red_bg;
                    }
                }
            }

            let glyphs: Vec<_> = font.layout(&count_str, font_scale, rusttype::point(0.0, 0.0)).collect();
            let text_width = glyphs.iter()
                .filter_map(|g| g.pixel_bounding_box())
                .map(|bb| bb.max.x)
                .max()
                .unwrap_or(0) as f32;

            let text_x = badge_x as f32 + (badge_size as f32 - text_width) / 2.0;
            let text_y = badge_y as f32 + (badge_size as f32 / 2.0) + (v_metrics.ascent / 2.0) - 4.0;

            let centered_glyphs = font.layout(&count_str, font_scale, rusttype::point(text_x, text_y));
            for g in centered_glyphs {
                if let Some(bb) = g.pixel_bounding_box() {
                    g.draw(|px, py, intensity| {
                        let tx = (px as i32 + bb.min.x) as usize;
                        let ty = (py as i32 + bb.min.y) as usize;
                        if intensity > 0.4 && tx < theme.width() && ty < pocket_h {
                            view[(ty, tx)] = white_text;
                        }
                    });
                }
            }
        }

        x_offset += square_size;
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
) -> (usize, usize) {
    let bar_height = theme.bar_height();
    let scale = Scale {
        x: CLOCK_FONT_SIZE,
        y: CLOCK_FONT_SIZE,
    };
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
                let top = top as i32 + bb.min.y + (BAR_PADDING + v_metrics.ascent) as i32;
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

    let clock_left = theme.width() - region_width - CLOCK_REGION_PADDING;
    (region_width, clock_left)
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
) -> [(Option<u32>, usize); 2] {
    orientation.fold(
        [(frame.black_clock, 0), (frame.white_clock, btm_bar_y)],
        [(frame.white_clock, 0), (frame.black_clock, btm_bar_y)],
    )
}
