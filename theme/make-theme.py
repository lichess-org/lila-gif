import xml.etree.ElementTree as ET
import chess.svg

from chess.svg import SQUARE_SIZE


COLORS = [
    "#ffce9e", # light square
    "#d18b47", # dark square
    "#cdd16a", # highlighted light square
    "#aaa23b", # highlighted dark square
    "#262421", # dark background
    "#bababa", # text color
    "#bf811d", # title color
]


def make_theme(f):
    svg = ET.Element("svg", {
        "xmlns": "http://www.w3.org/2000/svg",
        "version": "1.1",
        "xmlns:xlink": "http://www.w3.org/1999/xlink",
        "viewBox": f"0 0 {SQUARE_SIZE * 8} {SQUARE_SIZE * 8}",
    })

    defs = ET.SubElement(svg, "defs")
    for g in chess.svg.PIECES.values():
        defs.append(ET.fromstring(g))

    defs.append(ET.fromstring(chess.svg.CHECK_GRADIENT))

    for x, color in enumerate(COLORS):
        ET.SubElement(svg, "rect", {
            "x": str(SQUARE_SIZE * x),
            "y": "0",
            "width": str(SQUARE_SIZE),
            "height": str(SQUARE_SIZE),
            "stroke": "none",
            "fill": color,
        })

    for x in range(8):
        ET.SubElement(svg, "rect", {
            "x": str(SQUARE_SIZE * x),
            "y": str(SQUARE_SIZE),
            "width": str(SQUARE_SIZE),
            "height": str(SQUARE_SIZE * 7),
            "stroke": "none",
            "fill": COLORS[x % 4],
        })

        for y in range(1, 8):
            piece_type = min(y, 6)
            color = "white" if x >= 4 else "black"

            if y == 7:
                ET.SubElement(svg, "rect", {
                    "x": str(SQUARE_SIZE * x),
                    "y": str(SQUARE_SIZE * y),
                    "width": str(SQUARE_SIZE),
                    "height": str(SQUARE_SIZE),
                    "fill": "url(#check_gradient)",
                })

            ET.SubElement(svg, "use", {
                "xlink:href": f"#{color}-{chess.PIECE_NAMES[piece_type]}",
                "transform": f"translate({SQUARE_SIZE * x}, {SQUARE_SIZE * y})",
            })

    f.write(ET.tostring(svg))


if __name__ == "__main__":
    make_theme(open("theme.svg", "wb"))
