/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use super::points::Point;
use super::shapes::Shape;

#[derive(Debug)]
pub struct Entity {
    shape: Shape,
    point: Point,
    position: EntityPosition,
}

impl Entity {
    pub fn new(shape: Shape, point: Point, position: EntityPosition) -> Entity {
        Entity {
            shape,
            point,
            position,
        }
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn position(&self) -> EntityPosition {
        self.position
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EntityPosition {
    Within,
    At,
}
