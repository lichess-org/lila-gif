use std::{borrow::Cow, vec};

use bytes::{buf::Writer, BufMut, Bytes, BytesMut};
use gif::{AnyExtension, DisposalMethod, Extension, Repeat};
use ndarray::{s, ArrayViewMut2};
use rusttype::{Font, Scale};
use shakmaty::{uci::Uci, Bitboard, Board};

use crate::{
    api::{Comment, Orientation, PlayerName, RequestBody, RequestParams},
    theme::{SpriteKey, Theme, Themes},
};

enum PlayerBar {
    Top,
    Bottom,
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

pub struct Render {
    theme: &'static Theme,
    font: &'static Font<'static>,
    buffer: Vec<u8>,
    comment: Option<Comment>,
    bars: Option<PlayerBars>,
    orientation: Orientation,
    frames: Vec<RenderFrame>,
    kork: bool,
}

impl Render {
    pub fn new_image(themes: &'static Themes, params: RequestParams) -> Render {
        let bars = params.white.is_some() || params.black.is_some();
        let theme = themes.get(params.theme, params.piece);
        Render {
            theme,
            font: themes.font(),
            buffer: vec![0; theme.height(bars) * theme.width()],
            comment: params.comment,
            bars: PlayerBars::from(params.white, params.black),
            orientation: params.orientation,
            frames: vec![RenderFrame {
                highlighted: highlight_uci(params.last_move),
                checked: params.check.to_square(&params.fen.0).into_iter().collect(),
                board: params.fen.0.board,
                delay: None,
            }],
            kork: false,
        }
    }

    pub fn new_animation(themes: &'static Themes, params: RequestBody) -> Render {
        let bars = params.white.is_some() || params.black.is_some();
        let default_delay = params.delay;
        let theme = themes.get(params.theme, params.piece);
        Render {
            theme,
            font: themes.font(),
            buffer: vec![0; theme.height(bars) * theme.width()],
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
                .collect(),
            kork: true,
        }
    }

    /// Generates the GIF, returning a Bytes object
    pub fn render(mut self) -> Bytes {
        let (image_height, image_width) = self.image_dims();

        // Create the GIF encoder and
        let mut encoder = gif::Encoder::new(
            BytesMut::new().writer(),
            image_width as u16,
            image_height as u16,
            self.theme.global_color_table(),
        )
        .expect("create encoder");

        encoder.set_repeat(Repeat::Infinite).expect("encode repeat");
        self.render_comment(&mut encoder);
        self.render_player_bar(PlayerBar::Top);
        self.render_player_bar(PlayerBar::Bottom);

        // Iterate over every RenderFrame and encode the diff between consecutive frames in the GIF
        for frame_index in 0..self.frames.len() {
            // Generates the diff frame and adds its contents to the buffer
            let (board_left, board_top, w, h) = self.render_frame_diff(frame_index);

            // For the first frame, the image block always begins at (0, 0) and spans the entire
            // width and height of the image. For all other frames, the diff must be positioned
            let (left, top, width, height) = if frame_index == 0 {
                (0, 0, image_width, image_height)
            } else {
                let block_top = board_top + self.bar_pixel_offset();
                (board_left, block_top, w, h)
            };

            encoder
                .write_frame(&gif::Frame {
                    delay: self.frames[frame_index].delay.unwrap_or(0),
                    dispose: DisposalMethod::Keep,
                    transparent: Option::Some(self.theme.transparent_color()),
                    needs_user_input: false,
                    top: top as u16,
                    left: left as u16,
                    width: width as u16,
                    height: height as u16,
                    interlaced: false,
                    palette: None,
                    buffer: Cow::Borrowed(&self.buffer),
                })
                .expect("write frame");
        }

        // Add a black frame at the end, to work around twitter cutting off the last frame.
        if self.kork {
            self.buffer.clear();
            self.buffer
                .resize(image_width * image_height, self.theme.bar_color());

            encoder
                .write_frame(&gif::Frame {
                    delay: 1,
                    dispose: DisposalMethod::Keep,
                    transparent: Option::Some(self.theme.transparent_color()),
                    needs_user_input: false,
                    top: 0,
                    left: 0,
                    width: image_width as u16,
                    height: image_height as u16,
                    interlaced: false,
                    palette: None,
                    buffer: Cow::Borrowed(&self.buffer),
                })
                .expect("write frame");
        }

        encoder
            .into_inner()
            .expect("add trailer")
            .get_ref()
            .to_owned()
            .freeze()
    }

    /// Encodes a comment block. If no comment was requested, the repo URL is used
    fn render_comment(&mut self, encoder: &mut gif::Encoder<Writer<BytesMut>>) {
        let comment = self
            .comment
            .as_ref()
            .map_or("https://github.com/lichess-org/lila-gif".as_bytes(), |c| {
                c.as_bytes()
            });

        let extension = AnyExtension::from(Extension::Comment);
        encoder
            .write_raw_extension(extension, &[comment])
            .expect("write comment");
    }

    /// Renders a single player bar (either top or bottom)
    fn render_player_bar(&mut self, player_bar: PlayerBar) {
        if self.bars.is_none() {
            return;
        }

        let bars = self.bars.as_ref().unwrap();
        let width = self.theme.width();
        let height = self.theme.height(true);
        let bar_height = self.theme.bar_height();

        let mut view = ArrayViewMut2::from_shape((height, width), &mut self.buffer).expect("shape");
        let mut view = view.slice_mut(match player_bar {
            PlayerBar::Bottom => s!((bar_height + width).., ..),
            PlayerBar::Top => s!(..bar_height, ..),
        });

        view.fill(self.theme.bar_color());

        let mut text_color = self.theme.text_color();
        let player_name = match player_bar {
            PlayerBar::Bottom => self.orientation.fold(&bars.white, &bars.black),
            PlayerBar::Top => self.orientation.fold(&bars.black, &bars.white),
        };

        if player_name.starts_with("BOT ") {
            text_color = self.theme.bot_color();
        } else {
            for title in &[
                "GM ", "WGM ", "IM ", "WIM ", "FM ", "WFM ", "NM ", "CM ", "WCM ", "WNM ", "LM ",
                "BOT ",
            ] {
                if player_name.starts_with(title) {
                    text_color = self.theme.gold_color();
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

        let v_metrics = self.font.v_metrics(scale);
        let glyphs = self.font.layout(
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
                        && left < self.theme.width() as i32
                        && 0 <= top
                        && top < self.theme.bar_height() as i32
                        && intensity >= 0.01
                    {
                        if intensity < 0.5 && text_color == self.theme.text_color() {
                            view[(top as usize, left as usize)] = self.theme.med_text_color();
                        } else {
                            view[(top as usize, left as usize)] = text_color;
                        }
                    }
                });
            } else {
                text_color = self.theme.text_color();
            }
        }
    }

    /// Renders the diff between consecutive frames. The `frame_index` parameter should be in the
    /// range [0, num_frames) and indicates which frame is being transitioned to. If `frame_index`
    /// is zero, there is no previous frame and the entire board will be rendered; otherwise, only
    /// the diff between the previous frame and the current frame is rendered.
    fn render_frame_diff(&mut self, frame_index: usize) -> (usize, usize, usize, usize) {
        let frame = &self.frames[frame_index];

        // When rendering the first frame, we must be careful not to overwrite the initial buffer
        // space used to render the top player bar. The `bar_offset` variable provides for this.
        let (prev, bar_offset) = if frame_index == 0 {
            (None, self.bar_buffer_offset())
        } else {
            (Some(&self.frames[frame_index - 1]), 0)
        };

        // Determine the min/max x and y coords involved in this frame
        let diff = prev.map_or(Bitboard::FULL, |p| p.diff(frame));
        let x_coords: Vec<_> = diff.into_iter().map(|sq| self.orientation.x(sq)).collect();
        let y_coords: Vec<_> = diff.into_iter().map(|sq| self.orientation.y(sq)).collect();
        let x_min = x_coords.iter().min().unwrap_or(&0);
        let x_max = x_coords.iter().max().unwrap_or(&0) + 1;
        let y_min = y_coords.iter().min().unwrap_or(&0);
        let y_max = y_coords.iter().max().unwrap_or(&0) + 1;

        let sq_len = self.theme.square();
        let width = (x_max - x_min) * sq_len;
        let height = (y_max - y_min) * sq_len;

        // We want a slice of unused buffer, with the proper width and height dimensions. Leave
        // the start of the buffer alone if it was used to render the top player bar.
        let mut view = ArrayViewMut2::from_shape((height, width), &mut self.buffer[bar_offset..])
            .expect("shape");

        // Every square in the grid starts off transparent...
        if prev.is_some() {
            view.fill(self.theme.transparent_color());
        }

        // ...and those squares which change in the current frame are then rendered
        for sq in diff {
            let key = SpriteKey {
                piece: frame.board.piece_at(sq),
                dark_square: sq.is_dark(),
                highlight: frame.highlighted.contains(sq),
                check: frame.checked.contains(sq),
            };

            let left = (self.orientation.x(sq) - x_min) * sq_len;
            let top = (self.orientation.y(sq) - y_min) * sq_len;

            view.slice_mut(s!(top..(top + sq_len), left..(left + sq_len)))
                .assign(&self.theme.sprite(key));
        }

        (sq_len * x_min, sq_len * y_min, width, height)
    }

    /// Returns the buffer size required to render one player bar
    fn bar_buffer_offset(&self) -> usize {
        if self.bars.is_some() {
            self.theme.width() * self.theme.bar_height()
        } else {
            0
        }
    }

    /// Returns the number of pixels required to render one player bar.
    fn bar_pixel_offset(&self) -> usize {
        if self.bars.is_some() {
            self.theme.bar_height()
        } else {
            0
        }
    }

    /// Returns a tuple of (image height, image width)
    fn image_dims(&self) -> (usize, usize) {
        (self.theme.height(self.bars.is_some()), self.theme.width())
    }
}

fn highlight_uci(uci: Option<Uci>) -> Bitboard {
    match uci {
        Some(Uci::Normal { from, to, .. }) => Bitboard::from(from) | Bitboard::from(to),
        Some(Uci::Put { to, .. }) => Bitboard::from(to),
        _ => Bitboard::EMPTY,
    }
}
