#!/usr/bin/env python3

import subprocess
import io
from PIL import Image, ImageDraw


SQUARE_SIZE = 90

HIGHLIGHT = "#9bc70069"

BOARD_THEMES = {
    "blue":   ("#dee3e6", "#8ca2ad"),
    "brown":  ("#f0d9b5", "#b58863"),
    "green":  ("#ffffdd", "#86a666"),
    "ic":     ("#ececec", "#c1c18e"),
    "pink":   ("#f1f1c9", "#f07272"),
    "purple": ("#9f90b0", "#7d4a8d"),
}

PIECE_SETS = [
    "alpha",
    "anarcandy",
    "caliente",
    "california",
    "cardinal",
    "cburnett",
    "celtic",
    "chess7",
    "chessnut",
    "cooke",
    "companion",
    "disguised",
    "dubrovny",
    "fantasy",
    "fresca",
    "gioco",
    "governor",
    "horsey",
    "icpieces",
    "kiwen-suwi",
    "kosal",
    "leipzig",
    "letter",
    "libra",
    "maestro",
    "merida",
    "monarchy",
    "mpchess",
    "pirouetti",
    "pixel",
    "reillycraig",
    "riohacha",
    "shapes",
    "spatial",
    "staunty",
    "tatiana",
]

NONTHEME_COLORS = [
    "#262421",   # dark background
    "#bababa",   # text color
    "#bf811d",   # title color
    "#b72fc6",   # bot color
    "#706f6e",   # 50% text color on dark background
    "#ffffff00", # transparency
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

def make_sprite(light, dark, pieces, check_gradient):
    image = Image.new("RGB", (8 * SQUARE_SIZE, 8 * SQUARE_SIZE))
    draw = ImageDraw.Draw(image, "RGBA")

    for x in range(8):
        # Background
        fill = light if x % 2 == 0 else dark
        rect = (x * SQUARE_SIZE, 0, (x + 1) * SQUARE_SIZE - 1, SQUARE_SIZE * 8 - 1)
        draw.rectangle(rect, fill=fill)
        if x in [2, 3, 6, 7]:
            draw.rectangle(rect, fill=HIGHLIGHT)

        # Pieces
        color = "b" if x < 4 else "w"
        for i, piece in enumerate("PNBRQKK"):
            y = i + 1
            pos = (x * SQUARE_SIZE, y * SQUARE_SIZE)

            if y == 7:
                image.paste(check_gradient, pos, check_gradient)

            piece = pieces[f"{color}{piece}"]
            image.paste(piece, pos, piece)

    image = image.convert("RGBA")
    draw = ImageDraw.Draw(image, "RGBA")

    for i, color in enumerate(NONTHEME_COLORS):
        width = 4 * SQUARE_SIZE / len(NONTHEME_COLORS)
        draw.rectangle((4 * SQUARE_SIZE + i * width, 0, 4 * SQUARE_SIZE + (i + 1) * width - 1, SQUARE_SIZE - 1), fill=color)

    return image.quantize(64, dither=0)

def main():
    check_gradient = resvg("check-gradient.svg")
    piece_sets = {piece_set: resvg_pieces(piece_set) for piece_set in PIECE_SETS}

    for board_theme, (light, dark) in BOARD_THEMES.items():
        print(f"Generating sprites for {board_theme}...")
        for piece_set, pieces in piece_sets.items():
            image = make_sprite(light=light, dark=dark, pieces=pieces, check_gradient=check_gradient)
            image.save(f"sprites/{board_theme}-{piece_set}.gif", optimize=True, interlace=False, transparency=image.getpixel((SQUARE_SIZE * 8 - 1, 0)))


if __name__ == "__main__":
    main()
