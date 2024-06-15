# lila-gif

[![Test](https://github.com/lichess-org/lila-gif/actions/workflows/test.yml/badge.svg)](https://github.com/lichess-org/lila-gif/actions/workflows/test.yml)

Webservice to render Gifs of chess positions and games, and stream them
frame by frame.

![Example: DrDrunkenstein vs. Zhigalko_Sergei](/example.gif)

| size    | render time | frames | colors | width  | height |
| ------- | ----------- | ------ | ------ | ------ | ------ |
| 336 KiB | ~60 ms      | 93     | 63     | 720 px | 840 px |

## Usage

```
lila-gif

USAGE:
    lila-gif [OPTIONS]

OPTIONS:
        --bind <BIND>    Listen on this address [default: 127.0.0.1:6175]
    -h, --help           Print help information
```

## HTTP API

### `GET /image.gif`

```
curl http://localhost:6175/image.gif?fen=4k3/6KP/8/8/8/8/7p/8 --output image.gif
```

| name        | type  | default                                   | description                                                                                  |
| ----------- | ----- | ----------------------------------------- | -------------------------------------------------------------------------------------------- |
| **fen**     | ascii | _starting position_                       | FEN of the position. Board part is sufficient.                                               |
| white       | utf-8 | _none_                                    | Name of the white player. Known chess titles are highlighted. Limited to 100 bytes.          |
| black       | utf-8 | _none_                                    | Name of the black player. Known chess titles are highlighted. Limited to 100 bytes.          |
| comment     | utf-8 | `https://github.com/lichess-org/lila-gif` | Comment to be added to GIF meta data. Limited to 255 bytes.                                  |
| lastMove    | ascii | _none_                                    | Last move in UCI notation (like `e2e4`).                                                     |
| check       | ascii | _none_                                    | Square of king in check (like `e1`).                                                         |
| orientation |       | `white`                                   | Pass `black` to flip the board.                                                              |
| theme       |       | `brown`                                   | Board theme. `blue`, `brown`, `green`, `ic`, `pink`, or `purple`.                            |
| piece       |       | `cburnett`                                | Piece set from this [list](https://github.com/lichess-org/lila-gif/tree/master/theme/piece). |

### `POST /game.gif`

```javascript
{
  "white": "Molinari", // optional
  "black": "Bordais", // optional
  "comment": "https://www.chessgames.com/perl/chessgame?gid=1251038", // optional
  "orientation": "white", // default
  "theme": "brown", // default
  "piece": "cburnett", // default
  "delay": 50, // default frame delay in centiseconds
  "frames": [
    // [...]
    {
      "fen": "r1bqkb1r/pp1ppppp/5n2/2p5/2P1P3/2Nn2P1/PP1PNP1P/R1BQKB1R w KQkq - 1 6",
      "delay": 500, // optionally overwrite default delay
      "lastMove": "b4d3", // optionally highlight last move
      "check": "e1" // optionally highlight king
    }
  ]
}
```

### `GET /example.gif`

```
curl http://localhost:6175/example.gif --output example.gif
```

Render an [example game](https://lichess.org/Q0iQs5Zi).

## Technique

Instead of rendering vector graphics at runtime, all pieces are prerendered
on every possible background. This allows preparing a minimal color palette
ahead of time. (Pieces are not just black and white, but need other colors
for anti-aliasing on the different background colors).

![Sprite](/theme/sprites/brown-cburnett.gif)

All thats left to do at runtime, is copying sprites and Gif encoding.
More than 95% of the rendering time is spent in LZW compression.

For animated games, frames only contain the changed squares on transparent
background. The example below is the last frame of the animation.

![Example frame](/example-frame.gif)

## License

lila-gif is licensed under the GNU Affero General Public License, version 3 or
any later version, at your option.

The generated images include text in
[Noto Sans](https://fonts.google.com/specimen/Noto+Sans) (Apache License 2.0)
and a pieces sets with
[various licenses](https://github.com/lichess-org/lila/blob/master/COPYING.md).
