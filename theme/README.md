# Make sprites

## Installation

```bash
pip install -r requirements.txt
```

You will also need to install the CLI tool [`resvg`](https://github.com/RazrFalcon/resvg):

```bash
cargo install resvg
```

## Input assets

### Piece sets

There are piece SVGs in the `theme/piece` directory, organized by set, originally from `lila`. When piece sets are added or updated in `lila` ([`public/piece`](https://github.com/lichess-org/lila/tree/master/public/piece)), the `theme/piece` directory should be updated with those changes.

### Board themes

We currently only create sprites for board themes that are SVGs - not for those that are image based. We only need two colors from the board theme SVGs, so they are hardcoded in `make-sprites.py`. Board theme SVGs are located in `lila` in [`public/images/board/svg`](https://github.com/lichess-org/lila/tree/master/public/images/board/svg).

## Regenerating sprites

```bash
python3 make-sprites.py
```

This command will use `resvg` to generate many GIFs in the `theme/sprites` directory, of the form `{boardtheme}-{pieceset}.gif`.

## Todo

- Create sprites for non-SVG board themes
