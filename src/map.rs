/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use super::entities::{Entity, EntityPosition};
use super::graph::{Graph, NodeHandle};
use super::points::Point;
use super::shapes::Shape;
use super::svg::{Colour, SvgBuilder};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    width: usize,
    height: usize,
    graph: Graph<usize>,
    point_nodes: HashMap<usize, NodeHandle>,
    entities: Vec<Entity>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut graph = Graph::new();
        let mut point_nodes = HashMap::new();
        for i in 0..((width + 1) * (height + 1)) {
            let h = graph.add_node(i);
            point_nodes.insert(i, h);
        }
        Map {
            width,
            height,
            graph,
            point_nodes,
            entities: Vec::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn connect(&mut self, p1: Point, p2: Point) {
        let h1 = *self.find_node(p1);
        let h2 = *self.find_node(p2);
        self.graph.add_edge(h1, h2);
    }

    pub fn disconnect(&mut self, p1: Point, p2: Point) {
        let h1 = *self.find_node(p1);
        let h2 = *self.find_node(p2);
        self.graph.remove_edge(h1, h2);
    }

    fn find_node(&self, p: Point) -> &NodeHandle {
        let key = p.x() + (self.width + 1) * p.y();
        match self.point_nodes.get(&key) {
            Some(h) => h,
            None => panic!("Point is not within map {:?}", p),
        }
    }

    pub fn are_connected(&self, p1: Point, p2: Point) -> bool {
        let h1 = *self.find_node(p1);
        let h2 = *self.find_node(p2);
        self.graph.is_edge_between(h1, h2)
    }

    fn contains_point(&self, p: Point) -> bool {
        p.x() <= self.width && p.y() <= self.height
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}

pub fn map_to_svg(map: &Map, dim: usize) -> String {
    let drawing = SvgMapDrawing::new(dim, map);
    drawing.draw(map)
}

struct SvgMapDrawing {
    builder: SvgBuilder,
    dim: usize,
}

impl SvgMapDrawing {
    fn new(dim: usize, map: &Map) -> SvgMapDrawing {
        let svg_width = dim * map.width();
        let svg_height = dim * map.height();
        SvgMapDrawing {
            dim,
            builder: SvgBuilder::new(svg_width, svg_height),
        }
    }

    fn draw(mut self, map: &Map) -> String {
        for x in 0..(map.width() + 1) {
            for y in 0..(map.height() + 1) {
                let point = Point::new(x, y);
                let right = Point::new(x + 1, y);
                let down = Point::new(x, y + 1);
                if map.contains_point(right) {
                    self = self.line(map, point, right);
                }
                if map.contains_point(down) {
                    self = self.line(map, point, down);
                }
            }
        }
        for entity in map.entities().iter() {
            match entity.shape() {
                Shape::Circle(radius) => {
                    let (x, y, r) = match entity.position() {
                        EntityPosition::Within => {
                            let x = entity.point().x() * self.dim + self.dim / 2;
                            let y = entity.point().y() * self.dim + self.dim / 2;
                            let r = self.dim / 2 - 1;
                            (x, y, r)
                        }
                        EntityPosition::At => {
                            let x = entity.point().x() * self.dim;
                            let y = entity.point().y() * self.dim;
                            (x, y, radius * self.dim)
                        }
                    };

                    self.builder = self.builder.circle(x, y, r, Colour::Black);
                }
            }
        }
        self.builder.build()
    }

    fn line(mut self, map: &Map, p1: Point, p2: Point) -> Self {
        let colour = if map.are_connected(p1, p2) {
            Colour::Black
        } else {
            Colour::Rgb(200, 200, 200)
        };
        self.builder = self.builder.line(
            p1.x() * self.dim,
            p1.y() * self.dim,
            p2.x() * self.dim,
            p2.y() * self.dim,
            colour,
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_map() {
        let width = 3;
        let height = 2;
        let map = Map::new(width, height);
        assert_eq!(map.width, 3);
        assert_eq!(map.height, 2);
    }

    #[test]
    fn test_connect_points() {
        let mut map = Map::new(3, 2);
        let p1 = point(1, 1);
        let p2 = point(1, 2);
        map.connect(p1, p2);
        assert!(map.are_connected(p1, p2));
    }

    #[test]
    fn test_disconnect_points() {
        let mut map = Map::new(3, 2);
        let p1 = point(1, 1);
        let p2 = point(1, 2);
        map.connect(p1, p2);
        map.disconnect(p1, p2);
        assert!(!map.are_connected(p1, p2));
        assert!(!map.are_connected(point(1, 1), point(2, 1)));
    }

    #[test]
    fn test_point_is_in_map() {
        let map = Map::new(5, 5);
        let valid_point = point(2, 2);
        assert!(map.contains_point(valid_point));
    }

    fn point(x: usize, y: usize) -> Point {
        Point::new(x, y)
    }
}