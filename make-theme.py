import chess.svg

import xml.etree.ElementTree as ET

# piece type x
# w white
# l light
# m move
# c check

SQUARE_SIZE = 90

def make_theme(f):
    svg = ET.Element("svg", {
        "xmlns": "http://www.w3.org/2000/svg",
        "version": "1.1",
        "xmlns:xlink": "http://www.w3.org/1999/xlink",
        "viewBox": f"0 0 {SQUARE_SIZE * 8} {SQUARE_SIZE * 9}",
    })

    defs = ET.SubElement(svg, "defs")
    for g in chess.svg.PIECES.values():
        defs.append(ET.fromstring(g))

    ET.SubElement(svg, "rect", {
        "x": "0",
        "y": "0",
        "width": str(2 * SQUARE_SIZE),
        "height": str(SQUARE_SIZE * 8),
        "stroke": "none",
        "fill": "#ffce9e",
    })

    ET.SubElement(svg, "rect", {
        "x": str(2 * SQUARE_SIZE),
        "y": "0",
        "width": str(2 * SQUARE_SIZE),
        "height": str(SQUARE_SIZE * 8),
        "stroke": "none",
        "fill": "#d18b47",
    })

    ET.SubElement(svg, "rect", {
        "x": str(4 * SQUARE_SIZE),
        "y": "0",
        "width": str(2 * SQUARE_SIZE),
        "height": str(SQUARE_SIZE * 8),
        "stroke": "none",
        "fill": "#cdd16a",
    })

    ET.SubElement(svg, "rect", {
        "x": str(6 * SQUARE_SIZE),
        "y": "0",
        "width": str(2 * SQUARE_SIZE),
        "height": str(SQUARE_SIZE * 8),
        "stroke": "none",
        "fill": "#aaa23b",
    })

    scale = SQUARE_SIZE / chess.svg.SQUARE_SIZE

    for row in range(1, 8):
        if row == 7:
            piece_type = 6
        else:
            piece_type = row

        ET.SubElement(svg, "use", {
            "xlink:href": f"#white-{chess.PIECE_NAMES[piece_type]}",
            "transform": f"translate(0, {SQUARE_SIZE * row}), scale({scale}, {scale})",
        })

        ET.SubElement(svg, "use", {
            "xlink:href": f"#black-{chess.PIECE_NAMES[piece_type]}",
            "transform": f"translate({SQUARE_SIZE}, {SQUARE_SIZE * row}), scale({scale}, {scale})",
        })

    ET.SubElement(svg, "rect", {
        "x": "0",
        "y": str(SQUARE_SIZE * 8),
        "width": str(SQUARE_SIZE),
        "height": str(SQUARE_SIZE),
        "stroke": "none",
        "fill": "#000",
    })

    f.write(ET.tostring(svg))

if __name__ == "__main__":
    make_theme(open("theme.svg", "wb"))
