# svg2gcode

[![Build Status](https://travis-ci.com/sameer/svg2gcode.svg?branch=master)](https://travis-ci.com/sameer/svg2gcode)

Convert any SVG 1.1 path to gcode for a pen plotter, laser engraver, etc.

## TODO

Please see the `todo.txt` file for the current plan for development.

The file uses [standard todo.txt
formatting](https://github.com/todotxt/todo.txt).

## Demonstration

### Input

```bash
cargo run --release -- examples/Vanderbilt_Commodores_logo.svg
```

![Vanderbilt Commodores Logo](examples/Vanderbilt_Commodores_logo.svg)

### Output, rendered at [https://ncviewer.com]()

```bash
cat out.gcode
```

![Vanderbilt Commodores Logo Gcode](examples/Vanderbilt_Commodores_logo_gcode.png)

## FAQ / Interesting details

* Can I convert a PDF to gcode? Yes! Follow [this guide using Inkscape to convert a PDF to an SVG](https://en.wikipedia.org/wiki/Wikipedia:Graphics_Lab/Resources/PDF_conversion_to_SVG#Conversion_with_Inkscape)

* Are shapes, fill patterns supported? All objects can be converted to paths in Inkscape with `Object to Path` for use with this program. Not sure how practical fill patterns would be -- if you have ideas, feel free to open as issue or a PR.

* What about a generic PPD driver for using a plotter as a printer? I thought about doing something like this where you package ghostscript + inkscape + svg2gcode but since plotter dimensions and capabilities vary, this is an exercise left to the reader for now.

