use std::{io::Write, vec};

use bytes::{buf::Writer, BufMut, Bytes, BytesMut};
use shakmaty::{Bitboard, Color, Piece, Role};

use super::renderer::{highlight_uci, LocalPiece, RenderFrame, RenderState};
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
                render_defs(&mut output, &self.theme);

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

fn render_defs(output: &mut Writer<BytesMut>, theme: &SvgTheme) {
    output.write("
    <defs>
    <radialGradient id=\"check_gradient\" r=\"0.5\"><stop offset=\"0%\" stop-color=\"#ff0000\" stop-opacity=\"1.0\" /><stop offset=\"50%\" stop-color=\"#e70000\" stop-opacity=\"1.0\" /><stop offset=\"100%\" stop-color=\"#9e0000\" stop-opacity=\"0.0\" /></radialGradient>".as_bytes()).unwrap();
    for piece_color in Color::ALL {
        for piece_role in Role::ALL {
            let piece = LocalPiece::new(Piece {
                color: piece_color,
                role: piece_role,
            });
            let piece_sprite = theme.get_piece(*piece);
            let square_size = theme.square_size();
            output
                .write(
                    format!(
                        "<svg id=\"{piece}\" width=\"{square_size}\" height=\"{square_size}\">",
                    )
                    .as_bytes(),
                )
                .unwrap();
            output.write(piece_sprite).unwrap();
            output.write("</svg>".as_bytes()).unwrap();
        }
    }
    output.write("</defs>".as_bytes()).unwrap();
}

const BOARD: &'static str = "
<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:x=\"http://www.w3.org/1999/xlink\"
     viewBox=\"0 0 8 8\" shape-rendering=\"crispEdges\">
<g id=\"a\">
  <g id=\"b\">
    <g id=\"c\">
      <g id=\"d\">
        <rect width=\"1\" height=\"1\" fill=\"#f0d9b5\" id=\"e\"/>
        <use x=\"1\" y=\"1\" href=\"#e\" x:href=\"#e\"/>
        <rect y=\"1\" width=\"1\" height=\"1\" fill=\"#b58863\" id=\"f\"/>
        <use x=\"1\" y=\"-1\" href=\"#f\" x:href=\"#f\"/>
      </g>
      <use x=\"2\" href=\"#d\" x:href=\"#d\"/>
    </g>
    <use x=\"4\" href=\"#c\" x:href=\"#c\"/>
  </g>
  <use y=\"2\" href=\"#b\" x:href=\"#b\"/>
</g>
<use y=\"4\" href=\"#a\" x:href=\"#a\"/>
</svg>
"; 

fn render_chessboard(
    output: &mut Writer<BytesMut>,
    frame: &RenderFrame,
    theme: &SvgTheme,
    orientation: &Orientation,
) {
    println!("render_chessboard {:?} frame: {:?}", orientation, frame);
    output.write(BOARD.as_bytes()).unwrap();
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
                false => "#d6d77d",
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
                  "
                  <text x=\"{text_x}\" y=\"{text_y}\" fill=\"red\">{sq}</text>
                  ",
              )
              .as_bytes(),
          )
          .unwrap();
        if key.check {
            output
                .write(
                    format!(
                        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"url(#check_gradient)\" />",
                        x + (square_size / 2),
                        y + (square_size / 2),
                        square_size / 2
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
    }
}

fn render_pieces(
    output: &mut Writer<BytesMut>,
    frame: &RenderFrame,
    theme: &SvgTheme,
    orientation: &Orientation,
) {
    for (sq, piece) in frame.board.clone() {
        let piece = LocalPiece::new(piece);
        let square_size = theme.square_size();
        println!("render pieces {sq} {:?}", piece);

        let x = orientation.x(sq) * square_size;
        let y = orientation.y(sq) * square_size;
        let sprite = format!("<use href=\"#{piece}\" x=\"{x}\" y=\"{y}\" />",);
        output.write(sprite.as_bytes()).unwrap();
    }
}
