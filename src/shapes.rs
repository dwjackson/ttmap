/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use super::points::Point;

#[derive(Debug)]
pub struct Rect {
    point: Point,
    width: usize,
    height: usize,
    boolean_op: ShapeBoolean,
}

impl Rect {
    pub fn new(point: Point, width: usize, height: usize, boolean_op: ShapeBoolean) -> Rect {
        Rect {
            point,
            width,
            height,
            boolean_op,
        }
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn boolean_op(&self) -> ShapeBoolean {
        self.boolean_op
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeBoolean {
    Or,
    Xor,
}

#[derive(Debug)]
pub struct Line {
    orientation: LineOrientation,
    start: Point,
    length: usize,
    boolean_op: ShapeBoolean,
}

impl Line {
    pub fn new(
        orientation: LineOrientation,
        start: Point,
        length: usize,
        boolean_op: ShapeBoolean,
    ) -> Line {
        Line {
            orientation,
            start,
            length,
            boolean_op,
        }
    }

    pub fn orientation(&self) -> LineOrientation {
        self.orientation
    }

    pub fn start(&self) -> Point {
        self.start
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn boolean_op(&self) -> ShapeBoolean {
        self.boolean_op
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LineOrientation {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle(usize),
    Square,
    Stair,
}
