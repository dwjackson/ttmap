/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use crate::entities::{Entity, EntityPosition};
use crate::graph::{Graph, NodeHandle};
use crate::points::Point;
use crate::shapes::Shape;
use crate::svg::{Colour, SvgBuilder};
use std::collections::{HashMap, HashSet};

const LIGHT_GRAY: Colour = Colour::Rgb(200, 200, 200);

#[derive(Debug)]
pub struct Map {
    width: usize,
    height: usize,
    graph: Graph<Point>,
    point_nodes: HashMap<usize, NodeHandle>,
    entities: Vec<Entity>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut graph = Graph::new();
        let mut point_nodes = HashMap::new();
        for (i, p) in grid_points(width, height).enumerate() {
            let h = graph.add_node(p);
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
        let h1 = *self.find_node(p1).unwrap();
        let h2 = *self.find_node(p2).unwrap();
        self.graph.add_edge(h1, h2);
    }

    pub fn disconnect(&mut self, p1: Point, p2: Point) {
        let h1 = *self.find_node(p1).unwrap();
        let h2 = *self.find_node(p2).unwrap();
        self.graph.remove_edge(h1, h2);
    }

    fn find_node(&self, p: Point) -> Option<&NodeHandle> {
        let key = p.x() + (self.width + 1) * p.y();
        self.point_nodes.get(&key)
    }

    pub fn point_exists(&self, p: Point) -> bool {
        if p.x() > self.width || p.y() > self.height {
            return false;
        }
        self.find_node(p).is_some()
    }

    pub fn are_connected(&self, p1: Point, p2: Point) -> bool {
        let h1 = *self.find_node(p1).unwrap();
        let h2 = *self.find_node(p2).unwrap();
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
        // Draw the grid
        for i in 0..map.width() {
            for j in 0..map.height() {
                let p = Point::new(i, j);
                self = self.grid_cell(p);
            }
        }

        // Draw grid points that connect into polygons
        let cycles = map.graph.find_cycles();
        for cycle in cycles.iter() {
            let points: Vec<Point> = cycle
                .iter()
                .map(|h| *map.graph.find_node(*h).unwrap().data())
                .filter(|p| map.contains_point(*p))
                .map(|p| p.scale(self.dim))
                .collect();
            self = self.polygon(points);
        }

        // Draw grid points that connect only into lines, rather than polygons
        let polygon_points: HashSet<Point> = cycles
            .iter()
            .flatten()
            .map(|h| *map.graph.find_node(*h).unwrap().data())
            .collect();
        let connected_components = map.graph.connected_components();
        for cc in connected_components.iter().filter(|c| c.len() > 1) {
            let handles: Vec<NodeHandle> = cc
                .iter()
                .filter(|h| !polygon_points.contains(map.graph.find_node(**h).unwrap().data()))
                .copied()
                .collect();
            if handles.is_empty() {
                continue;
            }
            let endpoints: Vec<NodeHandle> = handles
                .iter()
                .filter(|h| map.graph.find_node(**h).unwrap().edge_count() == 1)
                .copied()
                .collect();
            for chunk in endpoints.chunks(2) {
                let start = chunk[0];
                let end = if chunk.len() == 1 {
                    // There is an odd number of edges so arbitrarily pick an edge to draw to
                    endpoints[0]
                } else {
                    chunk[1]
                };
                let path = map.graph.find_path(start, end).unwrap();
                let points: Vec<Point> = path
                    .iter()
                    .map(|h| *map.graph.find_node(*h).unwrap().data())
                    .collect();
                let points = scale_points(&points, self.dim);
                self = self.path(points);
            }
        }

        // Draw entities
        for entity in map.entities().iter() {
            match entity.shape() {
                Shape::Circle(radius) => {
                    let (x, y, r) = match entity.position() {
                        EntityPosition::Within => {
                            let mid = self.dim / 2;
                            let p = entity.point().scale(self.dim) + Point::new(mid, mid);
                            let r = mid - 1;
                            (p.x(), p.y(), r)
                        }
                        EntityPosition::At => {
                            let p = entity.point().scale(self.dim);
                            (p.x(), p.y(), radius * self.dim)
                        }
                    };

                    self.builder = self.builder.circle(x, y, r, Colour::Black);
                }
                Shape::Square => {
                    let side = self.dim * 3 / 5; // 60% of dim
                    let offset = (self.dim - side) / 2;
                    let delta = Point::new(offset, offset);
                    let p = entity.point().scale(self.dim) + delta;
                    self.builder = self.builder.rect(p, side, side, Colour::Black);
                }
                Shape::Stair => {
                    let height = self.dim * 3 / 5; // 60% of dim
                    let offset = (self.dim - height) / 2;
                    let delta = Point::new(offset, offset);
                    let riser = self.dim / 5; // 20% of dim
                    let origin = entity.point().scale(self.dim) + delta;
                    let points = [
                        (0, 2 * riser),
                        (0, 3 * riser),
                        (height, height),
                        (height, 0),
                        (2 * riser, 0),
                        (2 * riser, riser),
                        (riser, riser),
                        (riser, 2 * riser),
                    ]
                    .iter()
                    .map(|(x, y)| Point::new(*x, *y) + origin)
                    .collect::<Vec<Point>>();
                    self.builder = self.builder.polygon(points, Colour::Black);
                }
            }
        }
        self.builder.build()
    }

    fn grid_cell(mut self, p: Point) -> Self {
        self.builder = self
            .builder
            .rect(p.scale(self.dim), self.dim, self.dim, LIGHT_GRAY);
        self
    }

    fn polygon(mut self, points: Vec<Point>) -> Self {
        self.builder = self.builder.polygon(points, Colour::Black);
        self
    }

    fn path(mut self, points: Vec<Point>) -> Self {
        self.builder = self.builder.path(points, Colour::Black);
        self
    }
}

fn grid_points(width: usize, height: usize) -> PointsIter {
    PointsIter {
        x: 0,
        y: 0,
        x_max: width + 1,
        y_max: height + 1,
    }
}

fn scale_points(points: &[Point], scale_factor: usize) -> Vec<Point> {
    points.iter().map(|p| p.scale(scale_factor)).collect()
}

struct PointsIter {
    x: usize,
    y: usize,
    x_max: usize,
    y_max: usize,
}

impl Iterator for PointsIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x_max || self.y >= self.y_max {
            return None;
        }
        let point = Point::new(self.x, self.y);
        if self.x + 1 >= self.x_max {
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }
        Some(point)
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

    #[test]
    fn test_point_exists() {
        let map = Map::new(2, 2);
        assert!(!map.point_exists(Point::new(3, 1)));
    }

    fn point(x: usize, y: usize) -> Point {
        Point::new(x, y)
    }
}
