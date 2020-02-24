lila-gif
========

Webservice to render gifs of chess positions and games.

![Example: Molinari vs. Bordais](/example.gif)

metric | data
--- | ---
frames | 10
colors | 31
size | 53 KiB
width | 720 px
height | 840 px
render time | ~30 ms

Technique
---------

Instead of rendering vector graphics at runtime, all pieces are prerendered
on every possible background. This allows preparing a minimal color palette
ahead of time. (Pieces are not just black and white, but need other colors
for anti-aliasing on the different background colors).

![Sprite](/theme/sprite.gif)

All thats left to do at runtime, is copying sprites and Gif encoding.

For animated games, frames only contain the changed squares on transparent
background.

License
-------

lila-gif is licensed under the GNU Affero General Public License, version 3 or
any later version, at your option.

The generated images include text in
[Noto Sans](https://fonts.google.com/specimen/Noto+Sans) (Apache License 2.0)
and a piece set by
[Colin M.L. Burnett](https://en.wikipedia.org/wiki/User:Cburnett)
(GFDL or BSD or GPL).
