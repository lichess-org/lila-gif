use std::{io::Write, vec};

use bytes::{buf::Writer, BufMut, Bytes, BytesMut};
use shakmaty::Bitboard;

use super::renderer::{highlight_uci, RenderFrame, RenderState};
use crate::{
    api::{Orientation, RequestParams},
    renderer::renderer::SpriteKey,
    svg_theme::SvgTheme,
};

pub struct SVGRenderer {
    theme: SvgTheme,
    state: RenderState,
    orientation: Orientation,
    frames: vec::IntoIter<RenderFrame>,
}

impl SVGRenderer {
    pub fn new_image(params: RequestParams) -> SVGRenderer {
        SVGRenderer {
            theme: SvgTheme::new(params.piece),
            state: RenderState::Preamble,
            orientation: params.orientation,
            frames: vec![RenderFrame {
                highlighted: highlight_uci(params.last_move),
                checked: params.check.to_square(&params.fen.0).into_iter().collect(),
                board: params.fen.0.board,
                delay: None,
            }]
            .into_iter(),
        }
    }
}

impl Iterator for SVGRenderer {
    type Item = Bytes;

    fn next(&mut self) -> Option<Bytes> {
        let mut output = BytesMut::new().writer();

        let chessboard_size = self.theme.chessboard_size();
        let svg_preamble = format!("<svg viewBox=\"0 0 {chessboard_size} {chessboard_size}\" xmlns=\"http://www.w3.org/2000/svg\">");
        match self.state {
            RenderState::Preamble => {
                output.write(svg_preamble.as_bytes()).unwrap();
                let frame = self.frames.next().unwrap_or_default();
                render_chessboard(&mut output, &frame, &self.theme, &self.orientation);
                render_pieces(&mut output, &frame, &self.theme, &self.orientation);

                output.write("</svg>".as_bytes()).unwrap();
                self.state = RenderState::Complete;
            }
            RenderState::Frame(_) => {
                self.state = RenderState::Complete;
            }
            RenderState::Complete => {
                return None;
            }
        }
        Some(output.into_inner().freeze())
    }
}

fn render_chessboard(
    output: &mut Writer<BytesMut>,
    frame: &RenderFrame,
    theme: &SvgTheme,
    orientation: &Orientation,
) {
    println!("render_chessboard {:?}", orientation);
    for sq in Bitboard::FULL {
        let key = SpriteKey {
            piece: frame.board.piece_at(sq),
            dark_square: sq.is_dark(),
            highlight: frame.highlighted.contains(sq),
            check: frame.checked.contains(sq),
        };

        let square_color = match key.highlight {
            true => match key.dark_square {
                true => "#b9ae4a",
                false => "#d6d77d"
            },
            false => match key.dark_square {
                true => "#b58863",
                false => "#f0d9b5",
            },
        };

        let square_size = theme.square_size();

        let x = orientation.x(sq) * square_size;
        let y = orientation.y(sq) * square_size;
        println!("coords x: {x} y: {y} {sq} {square_color}");

        let text_x = x;
        let text_y = y + 80;

        output
          .write(
              format!(
                  "<rect x=\"{x}\" y=\"{y}\" width=\"{square_size}\" height=\"{square_size}\" fill=\"{square_color}\" />
                  <text x=\"{text_x}\" y=\"{text_y}\" fill=\"red\">{sq}</text>
                  ",
              )
              .as_bytes(),
          )
          .unwrap();
    }
}

fn render_pieces(
    output: &mut Writer<BytesMut>,
    frame: &RenderFrame,
    theme: &SvgTheme,
    orientation: &Orientation,
) {
    for (sq, piece) in frame.board.clone() {
        let square_size = theme.square_size();
        println!("render pieces {sq} {:?}", piece);
        let sprite = theme.get_piece(piece);

        let x = orientation.x(sq) * square_size;
        let y = orientation.y(sq) * square_size;
        output
            .write(
                format!(
                    "<svg x=\"{x}\" y=\"{y}\" width=\"{square_size}\" height=\"{square_size}\">"
                )
                .as_bytes(),
            )
            .unwrap();
        output.write(sprite).unwrap();
        output.write("</svg>".as_bytes()).unwrap();
    }
}
