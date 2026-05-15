#!/usr/bin/env python3

import os
import subprocess
import io

import numpy as np

from PIL import Image, ImageColor, ImageDraw


SQUARE_SIZE = 90

TRANSPARENCY = "#ffffff00"
HIGHLIGHT = "#9bc70069"
DARK_BACKGROUND = "#262421"
TEXT_COLOR = "#bababa"
TITLE_COLOR = "#bf811d"
BOT_COLOR = "#b72fc6"
GLYPH_TEXT_COLOR = "#ffffff"

BOARD_THEMES = {
    "blue":   ("#dee3e6", "#8ca2ad"),
    "brown":  ("#f0d9b5", "#b58863"),
    "green":  ("#ffffdd", "#86a666"),
    "ic":     ("#ececec", "#c1c18e"),
    "pink":   ("#f1f1c9", "#f07272"),
    "purple": ("#9f90b0", "#7d4a8d"),
}

NONTHEME_COLORS = [
    (TRANSPARENCY, TRANSPARENCY),
    (TEXT_COLOR, DARK_BACKGROUND), # Clock or player name on bar
    (TITLE_COLOR, DARK_BACKGROUND), # GM, WGM, ...
    (BOT_COLOR, DARK_BACKGROUND), # BOT
    (GLYPH_TEXT_COLOR, "#22ac38"), # !
    (GLYPH_TEXT_COLOR, "#168226"), # !!
    (GLYPH_TEXT_COLOR, "#e69f00"), # ?
    (GLYPH_TEXT_COLOR, "#df5353"), # ??
    (GLYPH_TEXT_COLOR, "#ea45d8"), # !?
    (GLYPH_TEXT_COLOR, "#56b4e9"), # ?!
    (GLYPH_TEXT_COLOR, "#a04048"), # □
    (GLYPH_TEXT_COLOR, "#9171f2"), # ⨀
]


def resvg(path):
    res = subprocess.run(
        ["resvg", path, "-c", "-w", "90"],
        stdout=subprocess.PIPE,
    )

    return Image.open(io.BytesIO(res.stdout), formats=["PNG"])


def resvg_pieces(piece_set):
    print(f"Preparing {piece_set} pieces...")
    return {f"{color}{piece}": resvg(f"piece/{piece_set}/{color}{piece}.svg") for color in "wb" for piece in "PNBRQK"}


def blend(bottom, top):
    b = Image.new('RGBA', (1, 1), ImageColor.getrgb(bottom))
    t = Image.new('RGBA', (1, 1), ImageColor.getrgb(top))
    r, g, b, _ = Image.alpha_composite(b, t).getpixel((0, 0))
    return f"#{r:02x}{g:02x}{b:02x}"


def make_sprite(light, dark, pieces, check_gradient):
    gradients = [(light, dark), (blend(light, HIGHLIGHT), blend(dark, HIGHLIGHT))] + NONTHEME_COLORS

    image = Image.new("RGB", (8 * SQUARE_SIZE, (7 + len(gradients)) * SQUARE_SIZE))
    draw = ImageDraw.Draw(image, "RGBA")

    for x in range(8):
        # Background
        fill = light if x % 2 == 0 else dark
        rect = (x * SQUARE_SIZE, 0, (x + 1) * SQUARE_SIZE - 1, SQUARE_SIZE * 8 - 1)
        draw.rectangle(rect, fill=blend(fill, HIGHLIGHT) if x in [2, 3, 6, 7] else fill)

        # Pieces
        color = "b" if x < 4 else "w"
        for y, piece in enumerate("PNBRQKK"):
            pos = (x * SQUARE_SIZE, y * SQUARE_SIZE)

            if y == 6:
                image.paste(check_gradient, pos, check_gradient)

            piece = pieces[f"{color}{piece}"]
            image.paste(piece, pos, piece)

    image = image.convert("RGBA")
    draw = ImageDraw.Draw(image, "RGBA")

    # Gradients for anti-aliased text
    for i, (left, right) in enumerate(gradients):
        left_color = np.array(ImageColor.getrgb(left) + (255, ))[:4]
        right_color = np.array(ImageColor.getrgb(right) + (255, ))[:4]
        t = np.linspace(0, 1, 8 * SQUARE_SIZE)[None, :, None]
        line = (left_color * (1 - t) + right_color * t).astype(np.uint8)
        rectangle = Image.fromarray(np.broadcast_to(line, (SQUARE_SIZE, 8 * SQUARE_SIZE, 4)), "RGBA")
        image.paste(rectangle, (0, (7 + i) * SQUARE_SIZE))

    return image.quantize(255, dither=0)

def main():
    check_gradient = resvg("check-gradient.svg")

    piece_dirs = [ os.path.basename(f.path) for f in os.scandir("piece") if f.is_dir() ]
    piece_sets = {piece_set: resvg_pieces(piece_set) for piece_set in piece_dirs}

    for board_theme, (light, dark) in BOARD_THEMES.items():
        print(f"Generating sprites for {board_theme}...")
        for piece_set, pieces in piece_sets.items():
            image = make_sprite(light=light, dark=dark, pieces=pieces, check_gradient=check_gradient)
            image.save(f"sprites/{board_theme}-{piece_set}.gif", optimize=True, interlace=False, transparency=image.getpixel((0, SQUARE_SIZE * 9)))

    rust_code_updates(piece_dirs)


def to_pascal_case(name: str) -> str:
    return "".join(x.capitalize() for x in name.lower().split("-"))


def rust_code_updates(piece_dirs):
    print("🦀 Update `src/assets.rs` with these:")
    print("#" * 80)

    rust_enum = "pub enum PieceSet {\n"
    for piece_set in sorted(piece_dirs):
        if piece_set == "cburnett":
            rust_enum += f"    #[default]\n"
        elif "-" in piece_set:
            rust_enum += f"    #[serde(rename = \"{piece_set}\")]\n"
        rust_enum += f"    {to_pascal_case(piece_set)},\n"
    rust_enum += "}"
    print(rust_enum)

    print("#" * 80)
    inner_data = "inner: [\n"
    for piece_set in sorted(piece_dirs):
        inner_data += f"    {to_pascal_case(piece_set)},\n"
    inner_data += "]"
    print(inner_data)

    print("#" * 80)
    colors = ["blue", "brown", "green", "ic", "pink", "purple"]
    sprite_data = "match board {"
    for color in colors:
        sprite_data += f"\n    BoardTheme::{color.capitalize()} => match pieces {{\n"
        for piece_set in sorted(piece_dirs):
            sprite_data += f"        {to_pascal_case(piece_set)} => include_bytes!(\"../theme/sprites/{color}-{piece_set}.gif\"),\n"
        sprite_data += "    },\n"
    sprite_data += "}"
    print(sprite_data)

if __name__ == "__main__":
    main()
