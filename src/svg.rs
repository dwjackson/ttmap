/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use crate::points::Point;

const SVG_XMLNS: &str = "http://www.w3.org/2000/svg";

pub struct SvgBuilder {
    height: usize,
    width: usize,
    elements: Vec<Box<dyn ToSvg>>,
}

trait ToSvg {
    fn to_svg(&self) -> String;
}

#[derive(Debug)]
struct SvgRect {
    point: Point,
    width: usize,
    height: usize,
    stroke: Colour,
}

impl ToSvg for SvgRect {
    fn to_svg(&self) -> String {
        format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" stroke=\"{}\" fill=\"none\"/>",
            self.point.x(),
            self.point.y(),
            self.width,
            self.height,
            self.stroke.to_svg()
        )
    }
}

#[derive(Debug)]
struct SvgPath {
    points: Vec<Point>,
    stroke: Colour,
}

impl ToSvg for SvgPath {
    fn to_svg(&self) -> String {
        let points_strs: Vec<String> = self
            .points
            .iter()
            .map(|p| format!("{} {}", p.x(), p.y()))
            .collect();
        let start = format!("M{}", points_strs[0]);
        let lines_strs: Vec<String> = points_strs
            .iter()
            .skip(1)
            .map(|s| format!("L{}", s))
            .collect();
        let lines_str = lines_strs.join(" ");
        format!(
            "<path d=\"{} {}\" stroke=\"{}\" fill=\"none\"/>",
            start,
            lines_str,
            self.stroke.to_svg()
        )
    }
}

#[derive(Debug)]
struct SvgCircle {
    x: usize,
    y: usize,
    radius: usize,
    stroke: Colour,
}

impl ToSvg for SvgCircle {
    fn to_svg(&self) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"{}\" fill=\"none\"/>",
            self.x,
            self.y,
            self.radius,
            self.stroke.to_svg(),
        )
    }
}

#[derive(Debug)]
struct SvgPolygon {
    points: Vec<Point>,
    stroke: Colour,
}

impl ToSvg for SvgPolygon {
    fn to_svg(&self) -> String {
        let points_strings: Vec<String> = self
            .points
            .iter()
            .map(|p| format!("{},{}", p.x(), p.y()))
            .collect();
        let points_str = points_strings.join(" ");
        format!(
            "<polygon points=\"{}\" stroke=\"{}\" fill=\"none\"/>",
            points_str,
            self.stroke.to_svg()
        )
    }
}

impl SvgBuilder {
    pub fn new(width: usize, height: usize) -> SvgBuilder {
        SvgBuilder {
            height,
            width,
            elements: Vec::new(),
        }
    }

    pub fn rect(mut self, point: Point, width: usize, height: usize, stroke: Colour) -> SvgBuilder {
        let rect = SvgRect {
            point,
            width,
            height,
            stroke,
        };
        self.elements.push(Box::new(rect));
        self
    }

    pub fn path(mut self, points: Vec<Point>, stroke: Colour) -> SvgBuilder {
        let path = SvgPath { points, stroke };
        self.elements.push(Box::new(path));
        self
    }

    pub fn circle(mut self, x: usize, y: usize, radius: usize, stroke: Colour) -> SvgBuilder {
        let circle = SvgCircle {
            x,
            y,
            radius,
            stroke,
        };
        self.elements.push(Box::new(circle));
        self
    }

    pub fn polygon(mut self, points: Vec<Point>, stroke: Colour) -> SvgBuilder {
        let polygon = SvgPolygon { points, stroke };
        self.elements.push(Box::new(polygon));
        self
    }

    pub fn build(&self) -> String {
        let mut svg = String::new();
        svg.push_str(&format!(
            "<svg version=\"1.1\" width=\"{}\" height=\"{}\" xmlns=\"{}\">",
            self.width, self.height, SVG_XMLNS
        ));
        for elem in self.elements.iter() {
            let elem_svg = elem.to_svg();
            svg.push_str(&elem_svg);
        }
        svg.push_str("</svg>");
        svg
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Colour {
    Black,
    Rgb(u8, u8, u8),
}

impl ToSvg for Colour {
    fn to_svg(&self) -> String {
        match self {
            Colour::Black => "black".to_string(),
            Colour::Rgb(r, g, b) => format!("rgb({r}, {g}, {b})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIDTH: usize = 300;
    const HEIGHT: usize = 200;

    #[test]
    fn test_empty_svg() {
        let builder = SvgBuilder::new(WIDTH, HEIGHT);
        let svg = builder.build();
        assert_eq!(
            "<svg version=\"1.1\" width=\"300\" height=\"200\" xmlns=\"http://www.w3.org/2000/svg\"></svg>",
            svg
        );
    }

    #[test]
    fn test_svg_with_rectangle() {
        let p = Point::new(10, 20);
        let builder = SvgBuilder::new(WIDTH, HEIGHT).rect(p, 100, 50, Colour::Black);
        let svg = builder.build();
        assert_eq!(
            "<svg version=\"1.1\" width=\"300\" height=\"200\" xmlns=\"http://www.w3.org/2000/svg\"><rect x=\"10\" y=\"20\" width=\"100\" height=\"50\" stroke=\"black\" fill=\"none\"/></svg>",
            svg
        );
    }

    #[test]
    fn test_svg_with_simple_path() {
        let points = vec![Point::new(50, 50), Point::new(100, 100)];
        let builder = SvgBuilder::new(WIDTH, HEIGHT).path(points, Colour::Rgb(200, 200, 200));
        let svg = builder.build();
        assert_eq!(
            "<svg version=\"1.1\" width=\"300\" height=\"200\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M50 50 L100 100\" stroke=\"rgb(200, 200, 200)\" fill=\"none\"/></svg>",
            svg
        );
    }

    #[test]
    fn test_circle() {
        let builder = SvgBuilder::new(WIDTH, HEIGHT).circle(100, 100, 20, Colour::Black);
        let svg = builder.build();
        assert_eq!(svg,
            "<svg version=\"1.1\" width=\"300\" height=\"200\" xmlns=\"http://www.w3.org/2000/svg\"><circle cx=\"100\" cy=\"100\" r=\"20\" stroke=\"black\" fill=\"none\"/></svg>");
    }

    #[test]
    fn test_polygon() {
        let points = vec![
            Point::new(100, 100),
            Point::new(120, 100),
            Point::new(120, 120),
        ];
        let builder = SvgBuilder::new(WIDTH, HEIGHT).polygon(points, Colour::Black);
        let svg = builder.build();
        assert_eq!(svg,  "<svg version=\"1.1\" width=\"300\" height=\"200\" xmlns=\"http://www.w3.org/2000/svg\"><polygon points=\"100,100 120,100 120,120\" stroke=\"black\" fill=\"none\"/></svg>");
    }
}
