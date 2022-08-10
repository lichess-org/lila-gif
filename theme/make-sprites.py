from chess.svg import SQUARE_SIZE
from genericpath import isdir
from os import listdir
from os.path import join
from PIL import Image
import chess.svg
import io
import re
import subprocess
import xml.etree.ElementTree as ET

# Highlight color defined in chessground:
# https://github.com/lichess-org/chessground/blob/b59f71127bec7151e4d929d6b66050c0b54ed6f7/assets/chessground.brown.css#L27
HIGHLIGHT_RGB = (155, 199, 0)
HIGHLIGHT_ALPHA = 0.41

# Calculate a highlighted theme color by blending the color with the semi transparent highlight color
# https://stackoverflow.com/questions/33371939/calculate-rgb-equivalent-of-base-colors-with-alpha-of-0-5-over-white-background
def blend_with_highlight(hex_color: str):
    rgb_color = tuple(int(hex_color[i : i + 2], 16) for i in (1, 3, 5))
    blended_rgb_color = tuple(
        round(HIGHLIGHT_ALPHA * c1 + (1 - HIGHLIGHT_ALPHA) * c2)
        for (c1, c2) in zip(HIGHLIGHT_RGB, rgb_color)
    )
    return "#%02x%02x%02x" % blended_rgb_color


def include_highlight_colors(theme: dict):
    theme["highlighted_light"] = blend_with_highlight(theme["light"])
    theme["highlighted_dark"] = blend_with_highlight(theme["dark"])
    return theme


# Board theme SVGs are stored in lila:
# https://github.com/lichess-org/lila/tree/master/public/images/board/svg
# All that is relevant are the light and dark square colors, which are copied/pasted here.
THEMES = {
    "blue": include_highlight_colors({"light": "#dee3e6", "dark": "#8ca2ad"}),
    "brown": include_highlight_colors({"light": "#f0d9b5", "dark": "#b58863"}),
    "green": include_highlight_colors({"light": "#ffffdd", "dark": "#86a666"}),
    "ic": include_highlight_colors({"light": "#ececec", "dark": "#c1c18e"}),
    "purple": include_highlight_colors({"light": "#9f90b0", "dark": "#7d4a8d"}),
}


def get_piece_set_names():
    return set([f for f in listdir("./piece") if isdir(join("./piece", f))])


NONTHEME_COLORS = [
    "#262421",  # dark background
    "#bababa",  # text color
    "#bf811d",  # title color
    "#b72fc6",  # bot color
    "#706f6e",  # 50% text color on dark background
]

COLOR_WIDTH = SQUARE_SIZE * 2 // 3

PIECE_TYPES = ["P", "N", "B", "R", "Q", "K"]
PIECES = ["bB", "bK", "bN", "bP", "bQ", "bR", "wB", "wK", "wN", "wP", "wQ", "wR"]

# Resize the piece SVG to SQUARE_SIZE
def resize_svg_root(root: ET.Element):
    if root.get("viewBox") is None:
        root.set("viewBox", "0 0 %s %s" % (root.attrib["width"], root.attrib["height"]))
    root.attrib["width"] = f"{SQUARE_SIZE}"
    root.attrib["height"] = f"{SQUARE_SIZE}"


# Ensure that the ids of all elements are unique by prefixing with the name of the piece
# Can be done with SVGO prefixIds instead
def namespace_ids(element: ET.Element, piece: str):
    prefix = piece + "-"
    if element.get("id"):
        element.set("id", prefix + element.get("id"))
    for key, value in element.attrib.items():
        id_index = value.find("url(#")
        if id_index >= 0:
            element.set(key, value[0 : id_index + 5] + prefix + value[id_index + 5 :])
        elif key.endswith("href") and value.startswith("#"):
            element.set(key, "#" + prefix + element.get(key)[1:])
    for child in element:
        namespace_ids(child, piece)


# Ensure that the class names of all elements are unique by prefixing with the name of the piece
# Can be done with SVGO prefixIds instead
def namespace_classnames(element: ET.Element, piece: str):
    prefix = piece + "-"
    if element.get("class"):
        element.set(
            "class",
            " ".join(
                [prefix + classname for classname in element.get("class").split(" ")]
            ),
        )
    # For any style tags like:
    # <style>.st0{fill:none}.st1{fill:#010101}.st2{fill:#6d6e6e}</style>
    # Include a piece prefix in front of every class name:
    # <style>.bB-st0{fill:none}.bB-st1{fill:#010101}.bB-st2{fill:#6d6e6e}</style>
    if element.tag.endswith("style"):
        element.text = re.sub(r"\.(.*?)\{", r"." + prefix + r"\1{", element.text)
    for child in element:
        namespace_classnames(child, piece)


piece_sets = {}


def make_piece_set(piece_set_name: str):
    if piece_sets.get(piece_set_name):
        return piece_sets[piece_set_name]

    piece_set = []
    for piece in PIECES:
        svg = ET.parse(f"piece/{piece_set_name}/{piece}.svg")

        root = svg.getroot()
        resize_svg_root(root)

        # TODO: run SVGO with the prefixIds plugin on all SVGs so we don't have to do the following
        namespace_ids(root, piece)
        namespace_classnames(root, piece)

        root.attrib["id"] = piece
        piece_set.append(ET.tostring(root, "utf8", method="xml"))

    piece_sets[piece_set_name] = piece_set
    return piece_set


def make_sprite(theme_name: str, piece_set_name: str):
    svg = ET.Element(
        "svg",
        {
            "xmlns": "http://www.w3.org/2000/svg",
            "version": "1.1",
            "xmlns:xlink": "http://www.w3.org/1999/xlink",
            "viewBox": f"0 0 {SQUARE_SIZE * 8} {SQUARE_SIZE * 8}",
        },
    )

    defs = ET.SubElement(svg, "defs")
    for g in make_piece_set(piece_set_name):
        defs.append(ET.fromstring(g))

    defs.append(ET.fromstring(chess.svg.CHECK_GRADIENT))

    for x, color in enumerate(NONTHEME_COLORS):
        ET.SubElement(
            svg,
            "rect",
            {
                "x": str(SQUARE_SIZE * 4 + COLOR_WIDTH * x),
                "y": "0",
                "width": str(COLOR_WIDTH),
                "height": str(SQUARE_SIZE),
                "stroke": "none",
                "fill": color,
            },
        )

    theme_colors = [
        THEMES[theme_name]["light"],
        THEMES[theme_name]["dark"],
        THEMES[theme_name]["highlighted_light"],
        THEMES[theme_name]["highlighted_dark"],
    ]

    for x in range(8):
        ET.SubElement(
            svg,
            "rect",
            {
                "x": str(SQUARE_SIZE * x),
                "y": str(SQUARE_SIZE if x >= 4 else 0),
                "width": str(SQUARE_SIZE),
                "height": str(SQUARE_SIZE * (7 if x >= 4 else 8)),
                "stroke": "none",
                "fill": theme_colors[x % 4],
            },
        )

        for y in range(1, 8):
            color = "w" if x >= 4 else "b"

            if y == 7:
                ET.SubElement(
                    svg,
                    "rect",
                    {
                        "x": str(SQUARE_SIZE * x),
                        "y": str(SQUARE_SIZE * y),
                        "width": str(SQUARE_SIZE),
                        "height": str(SQUARE_SIZE),
                        "fill": "url(#check_gradient)",
                    },
                )

            ET.SubElement(
                svg,
                "use",
                {
                    "xlink:href": f"#{color}{PIECE_TYPES[min(y, 6) - 1]}",
                    "transform": f"translate({SQUARE_SIZE * x}, {SQUARE_SIZE * y})",
                },
            )

    # Note: I originally implemented the SVG to PNG conversion with CairoSVG, but it proved unable to
    # render all of the SVGs properly (fantasy, spatial). Hence this somewhat hacky usage of librsvg.
    open("temp.svg", "wb").write(ET.tostring(svg))
    completed_process = subprocess.run(
        f"rsvg-convert -h {SQUARE_SIZE * 8 * 2} temp.svg",
        shell=True,
        capture_output=True,
    )

    image = Image.open(io.BytesIO(completed_process.stdout))
    print(f"sprites/{theme_name}-{piece_set_name}.gif")
    image.save(
        f"sprites/{theme_name}-{piece_set_name}.gif", optimize=True, interlace=False
    )


def make_all_sprites():
    for theme_name in THEMES.keys():
        for piece_set_name in get_piece_set_names():
            make_sprite(theme_name, piece_set_name)


if __name__ == "__main__":
    make_all_sprites()
