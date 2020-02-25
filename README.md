lila-gif
========

Webservice to render gifs of chess positions and games.

![Example: Molinari vs. Bordais](/example.gif)

size | render time | frames | colors | width | height
--- | --- | --- | --- | --- | ---
53 KiB | ~30 ms | 10 | 63 | 720 px | 840 px

HTTP API
--------

### `GET /image.gif`

```
curl http://localhost:3030/?fen=4k3/6KP/8/8/8/8/7p/8 --output image.gif
```

name | type | default | description
--- | --- | --- | ---
**fen** | string | *starting position* | FEN of the position. Board part is sufficient.
white | string | *none* | Username of white player. If multiple words, first word is assumed to be title.
black | string | *none* | Username of black player. If multiple words, first word is assumed to be title.
lastMove | string | *none* | Last move in UCI notation (like `e2e4`).
check | string | *none* | Square of king in check (like `e1`).
orientation | string | `white` | Pass `black` to flip the board.

### `POST /game.gif`

```javascript
{
  "white": "Molinari", // optional
  "black": "Bordais", // optional
  "orientation": "white", // default
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
curl http://localhost:3030/example.gif --output example.gif
```

Render the example Gif (Molinari vs. Bordais) for the README.

Technique
---------

Instead of rendering vector graphics at runtime, all pieces are prerendered
on every possible background. This allows preparing a minimal color palette
ahead of time. (Pieces are not just black and white, but need other colors
for anti-aliasing on the different background colors).

![Sprite](/theme/sprite.gif)

All thats left to do at runtime, is copying sprites and Gif encoding.

For animated games, frames only contain the changed squares on transparent
background. The example below is the last frame of the animation.

![Example frame](/example-frame.gif)

License
-------

lila-gif is licensed under the GNU Affero General Public License, version 3 or
any later version, at your option.

The generated images include text in
[Noto Sans](https://fonts.google.com/specimen/Noto+Sans) (Apache License 2.0)
and a piece set by
[Colin M.L. Burnett](https://en.wikipedia.org/wiki/User:Cburnett)
(GFDL or BSD or GPL).
