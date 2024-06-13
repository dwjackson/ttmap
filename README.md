<!--
This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

<!--
Copyright (c) 2024 David Jackson
-->

# ttmap 

TabletopMap--`ttmap`--is a program and a [DSL](https://en.wikipedia.org/wiki/Domain-specific_language)
for creating [TTRPG](https://en.wikipedia.org/wiki/Tabletop_role-playing_game) grid-based maps.

## Usage

The `ttmap` executable takes a map file name and, optionally, a grid-cell
dimension, in pixels. It will output an SVG to `stdout`.

```sh
$ ttmap [MAP_FILE] [DIMENSION?]
```

For example, if we have a map called `test.map` and we want each grid cell to
be 20px by 20px:

```sh
$ ttmap test.map 20 > test.svg
```

# The Language

Every map file must begin with a declaration of the size of the grid. Each
dimension is in the abstract unit of cells. After that, each line defines a
feature of the map.

Note that, in the syntax explanations, all strings contained within square
brackets are placeholders, to be replaced by real data in your actual files.

## Grid

The `grid` declaration defines the dimensions of the grid, in cells.

```txt
grid [WIDTH], [HEIGHT]
```

## Rectangles

To draw a rectangle, along grid lines, on the map, the `rect` command is used:

```txt
rect at [TOP_LEFT_X], [TOP_LEFT_Y] width [WIDTH] height [HEIGHT]
```

By default, all sides of the rectangle will be drawn. To draw more complex
shapes, rectangles can be combined. To remove "overlap" between two rectangles,
the `xor` command can be used.

```txt
xor rect at ...
```

To illustrate this, here is an example of what two combined rectangles could
look like:

```txt
+-----+
|     |
|      --+
|        |
+-----+--+
```

## Lines

To draw lines along grid edges, the `line` command is used:

```txt
line along [SIDE] from [X],[Y] length [LENGTH]
```

The valid sides are:

* `top`
* `bottom`
* `left`
* `right`

## Entities

Map entities are things "on" the map as opposed to _part of_ the map. They
are created with the `entity` command. Each entity has a shape and a position
within the map.

### Circles

To create a circle within a particular grid-square:

```txt
entity circle within [X], [Y]
```

To create a circle whose center point is at a grid-line intersection, with a
given radius:

```txt
entity circle at [X], [Y] radius [RADIUS] 
```

