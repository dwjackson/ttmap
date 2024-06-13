/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn down(&self) -> Point {
        Point::new(self.x, self.y + 1)
    }

    pub fn up(&self) -> Point {
        Point::new(self.x, self.y - 1)
    }

    pub fn right(&self) -> Point {
        Point::new(self.x + 1, self.y)
    }

    pub fn left(&self) -> Point {
        Point::new(self.x - 1, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_down() {
        let p = Point::new(2, 3);
        let p2 = p.down();
        assert_eq!(p2, Point::new(2, 4));
    }

    #[test]
    fn test_up() {
        let p = Point::new(2, 3);
        let p2 = p.up();
        assert_eq!(p2, Point::new(2, 2));
    }

    #[test]
    fn test_right() {
        let p = Point::new(2, 3);
        let p2 = p.right();
        assert_eq!(p2, Point::new(3, 3));
    }

    #[test]
    fn test_left() {
        let p = Point::new(2, 3);
        let p2 = p.left();
        assert_eq!(p2, Point::new(1, 3));
    }
}
