/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn scale(&self, scale_factor: usize) -> Point {
        Point::new(self.x * scale_factor, self.y * scale_factor)
    }

    pub fn taxicab_distance(&self, p: &Point) -> usize {
        let (x1, x2) = if self.x > p.x {
            (self.x, p.x)
        } else {
            (p.x, self.x)
        };
        let (y1, y2) = if self.y > p.y {
            (self.y, p.y)
        } else {
            (p.y, self.y)
        };
        (x1 - x2) + (y1 - y2)
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
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

    #[test]
    fn test_scale() {
        let p = Point::new(2, 3);
        let s = p.scale(10);
        assert_eq!(s.x(), 20);
        assert_eq!(s.y(), 30);
    }

    #[test]
    fn test_add() {
        let p1 = Point::new(2, 3);
        let p2 = Point::new(1, 2);
        let p3 = p1 + p2;
        assert_eq!(p3.x(), 3);
        assert_eq!(p3.y(), 5);
    }

    #[test]
    fn test_taxicab_distance_between_point_and_itself_is_zero() {
        let p = Point::new(0, 0);
        let d = p.taxicab_distance(&p);
        assert_eq!(d, 0);
    }

    #[test]
    fn test_taxicab_distance_between_horizontally_distanced_points() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(4, 1);
        let d = p1.taxicab_distance(&p2);
        assert_eq!(d, 3);
    }

    #[test]
    fn test_taxicab_distance_between_vertically_distanced_points() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(1, 4);
        let d = p1.taxicab_distance(&p2);
        assert_eq!(d, 3);
    }

    #[test]
    fn test_taxicab_distance() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(4, 4);
        let d = p1.taxicab_distance(&p2);
        assert_eq!(d, 6);
    }
}
