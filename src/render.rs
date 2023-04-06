use std::{io::Write, iter::FusedIterator, vec};

use bytes::{buf::Writer, BufMut, Bytes, BytesMut};
use gift::{block, Encoder};
use ndarray::{s, ArrayViewMut2};
use rusttype::{Font, LayoutIter, Scale};
use shakmaty::{uci::Uci, Bitboard, Board, File, Rank, Square};

use crate::{
    api::{Comment, Coordinates, Orientation, PlayerName, RequestBody, RequestParams},
    theme::{SpriteKey, Theme, Themes},
};

enum RenderState {
    Preamble,
    Frame(RenderFrame),
    Complete,
}

pub enum RenderFormat {
    GIF,
    SVG,
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

#[derive(Default, Debug)]
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

pub struct Render {
    render_format: RenderFormat,
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

impl Render {
    pub fn new_image(
        themes: &'static Themes,
        params: RequestParams,
        render_format: RenderFormat,
    ) -> Render {
        let bars = PlayerBars::from(params.white, params.black);
        let theme = themes.get(params.theme, params.piece);
        let buffer = match render_format {
            RenderFormat::GIF => vec![0; theme.height(bars.is_some()) * theme.width()],
            RenderFormat::SVG => vec![],
        };
        Render {
            render_format,
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

    pub fn new_animation(themes: &'static Themes, params: RequestBody) -> Render {
        let bars = PlayerBars::from(params.white, params.black);
        let default_delay = params.delay;
        let theme = themes.get(params.theme, params.piece);
        Render {
            render_format: RenderFormat::GIF,
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

const SVG_PREAMBLE: &'static str =
    "<svg viewBox=\"0 0 640 640\" xmlns=\"http://www.w3.org/2000/svg\"><rect width=\"100%\" height=\"100%\" fill=\"lightgray\"/>";

impl Iterator for Render {
    type Item = Bytes;

    fn next(&mut self) -> Option<Bytes> {
        let mut output = BytesMut::new().writer();
        match self.render_format {
            RenderFormat::GIF => {
                match self.state {
                    RenderState::Preamble => {
                        let mut blocks = Encoder::new(&mut output).into_block_enc();

                        blocks.encode(block::Header::default()).expect("enc header");

                        blocks
                            .encode(
                                block::LogicalScreenDesc::default()
                                    .with_screen_height(
                                        self.theme.height(self.bars.is_some()) as u16
                                    )
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

                        let comment =
                            self.comment.as_ref().map_or(
                                "https://github.com/lichess-org/lila-gif".as_bytes(),
                                |c| c.as_bytes(),
                            );
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
                                view.slice_mut(s!(
                                    (self.theme.bar_height() + self.theme.width())..,
                                    ..
                                )),
                                self.theme,
                                self.font,
                                self.orientation.fold(&bars.white, &bars.black),
                            );

                            view.slice_mut(s!(
                                self.theme.bar_height()
                                    ..(self.theme.bar_height() + self.theme.width()),
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
            }
            RenderFormat::SVG => match self.state {
                RenderState::Preamble => {
                    output.write(SVG_PREAMBLE.as_bytes()).unwrap();
                    let frame = self.frames.next().unwrap_or_default();
                    self.state = RenderState::Frame(frame);
                }
                RenderState::Frame(ref prev) => {
                    println!("{:?}", prev);
                    render_chessboard(&mut output);

                    output.write("</svg>".as_bytes()).unwrap();
                    self.state = RenderState::Complete;
                }
                RenderState::Complete => {
                    return None;
                }
            },
        }
        Some(output.into_inner().freeze())
    }
}

impl FusedIterator for Render {}

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

fn highlight_uci(uci: Option<Uci>) -> Bitboard {
    match uci {
        Some(Uci::Normal { from, to, .. }) => Bitboard::from(from) | Bitboard::from(to),
        Some(Uci::Put { to, .. }) => Bitboard::from(to),
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

const SQUARE_SIZE: u32 = 80;

fn render_chessboard(output: &mut Writer<BytesMut>) {
    println!("render_chessboard");
    for sq in Bitboard::FULL {
        let square_color = match sq.is_dark() {
            true => "black",
            false => "white",
        };

        let sprite = match sq.is_dark() {
            true => "<svg viewBox=\"0 0 2048 2048\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"m 553.33333,1485 -55,320 1047.33337,5 -47.3334,-335 c 0,0 323.3334,-313.3333 330,-466.6667 C 1835,855 1793.2385,677.36891 1586.6667,601.66667 1404.4831,534.90195 1215,723.33333 1215,723.33333 l -181.6667,-161.66666 -189.99997,160 c 0,0 -185.06774,-135.89105 -256.66666,-130 C 323.33333,613.33334 235,811.66667 225,925 c 10.16447,331.252 328.33333,560 328.33333,560 z\" fill=\"#f9f9f9\"/><path d=\"M1024 1769h489l-12-73H547l-12 73zm0-921q-25-60-62-111 31-48 62-65 30 17 62 65-38 51-62 111zm-97 454q-154 11-303 58-123-108-200-213.5T347 945q0-89 73.5-159T569 716q67 0 134.5 62.5T806 909q30 54 75 175t46 218zm-350 217l-26 156 145-84zm447-907q-47 0-136 121-31-36-50-55 93-140 186-140 92 0 186 140-20 19-50 55-90-121-136-121zm0 775q-1-126-42-267.5T898 893q-8-14-14-27t-12-23q-28-43-48-69-51-63-120-105t-134-42q-103 0-208 93T257 949q0 120 99 254.5T605 1463q201-74 419-76zm0 456H448l61-365q-325-280-326-535-1-159 125-274.5T575 553q78 0 158.5 47T876 719q61 74 98.5 164.5T1024 1034q12-60 49-150.5t99-164.5q61-72 142-119t159-47q140 0 266 115.5T1865 943q-2 255-326 535l61 365zm97-541q0-97 45-218t76-175q34-68 101.5-130.5T1479 716q74 0 147.5 70t74.5 159q0 96-77 201.5T1424 1360q-150-47-303-58zm350 217l-119 72 145 84zm-447-132q217 2 419 76 150-125 249-259.5t99-254.5q0-136-105.5-229T1478 627q-66 0-135 42t-119 105q-21 26-48 69-6 10-12.5 23l-13.5 27q-44 85-85 226.5t-41 267.5zm-139 159l139 86 139-84-139-86zm92-1248v-95h94v95h107v95h-107v153q-48-16-94 0V393H870v-95z\" fill=\"#101010\"/></svg>",
            false=> "<svg viewBox=\"0 0 2048 2048\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"m 501.64494,1810.9843 48.3236,-354.3731 -260.02697,-269.2314 c 0,0 -166.32924,-288.13389 29.91461,-480.93486 262.32809,-257.72584 506.24719,20.71012 506.24719,20.71012 l 195.59553,-165.6809 184.0899,165.6809 c 0,0 216.3056,-232.41349 430.3101,-75.93708 214.0045,156.4764 255.4247,317.55505 117.3573,531.55952 -138.0674,214.0045 -250.8225,280.7371 -250.8225,280.7371 l 55.227,347.4697 z\" fill=\"#f9f9f9\"/><path d=\"M977 298v-95h94v95h107v95h-107v153q-48-16-94 0V393H870v-95zm47 314q-47 0-136 121-31-36-50-55 93-140 186-140 92 0 186 140-20 19-50 55-90-121-136-121zm-447 907l-26 156 145-84zm410-206q-1-147-36.5-274.5T870 845q-45-88-131.5-153T570 627q-103 0-208 93T257 949q0 109 86.5 236T546 1408q212-88 441-95zm37 530H448l61-365q-325-280-326-535-1-159 125-274.5T575 553q78 0 158.5 47T876 719q61 74 98.5 164.5T1024 1034q12-60 49-150.5t99-164.5q61-72 142-119t159-47q140 0 266 115.5T1865 943q-2 255-326 535l61 365zm0-74h489l-50-298q-216-84-439-84t-439 84l-50 298zm447-250l26 156-145-84zm-410-206q229 7 441 95 115-96 202-223t87-236q0-136-105.5-229T1478 627q-83 0-169.5 65T1178 845q-46 66-81.5 193.5T1061 1313zm-176 233l141-84 137 86-141 84z\" fill=\"#101010\"/></svg>"
        };

        let x = ((sq.file().char() as u32) - (b'a' as u32)) * SQUARE_SIZE;
        let y = 640 - (((sq.rank().char() as u32) - (b'1' as u32) + 1) * SQUARE_SIZE);
        println!("coords x: {x} y: {y} {sq} {square_color}");

        let text_x = x;
        let text_y = y + 80;

        output
            .write(
                format!(
                    "<rect x=\"{x}\" y=\"{y}\" width=\"{SQUARE_SIZE}\" height=\"{SQUARE_SIZE}\" fill=\"{square_color}\" />
                    <svg  x=\"{x}\" y=\"{y}\" width=\"{SQUARE_SIZE}\" height=\"{SQUARE_SIZE}\">
                       {sprite}
                    </svg>
                    <text x=\"{text_x}\" y=\"{text_y}\" fill=\"red\">{sq}</text>
                    ",
                )
                .as_bytes(),
            )
            .unwrap();
    }
}
