lila-gif
========

WIP
---

* [x] request parsing
* [ ] single frame
* [ ] animations
* [ ] evaluation
* [ ] optimize gift
* [ ] write readme
* [ ] select theme
* [ ] cleanups

Theme
-----

Instead of rendering vector graphics at runtime, all pieces are prerendered
on every possible background. This allows preparing a minimal color palette
ahead of time. (Pieces are not just black and white, but need other colors
for anti-aliasing on the different backgrounds).

![Theme](/theme/theme.gif)

License
-------

lila-gif is licensed under the GNU Affero General Public License, version 3 or
any later version, at your option.

The generated images include text in
[Noto Sans](https://fonts.google.com/specimen/Noto+Sans) (Apache License 2.0)
and a piece set by
[Colin M.L. Burnett](https://en.wikipedia.org/wiki/User:Cburnett)
(GFDL or BSD or GPL).
