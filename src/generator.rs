/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use super::ast::{AbstractSyntaxTree, ShapeNode};
use super::entities::Entity;
use super::map::Map;
use super::points::Point;
use super::shapes::{Rect, ShapeBoolean};

#[derive(Debug)]
pub enum GridMapperGenerateError {
    NoGridDimensions,
}

pub fn generate_map(ast: &AbstractSyntaxTree) -> Result<Map, GridMapperGenerateError> {
    let dims = ast.grid_dimensions();
    if dims.is_none() {
        return Err(GridMapperGenerateError::NoGridDimensions);
    }
    let dims = dims.unwrap();

    let mut map = Map::new(dims.width(), dims.height());

    for shape in ast.shapes() {
        match shape {
            ShapeNode::Rect(rect) => handle_rect(&mut map, rect),
        }
    }

    for entity_node in ast.entities() {
        let entity = Entity::new(entity_node.shape, entity_node.point, entity_node.position);
        map.add_entity(entity);
    }

    Ok(map)
}

fn handle_rect(map: &mut Map, rect: &Rect) {
    // Connect all the points on the perimiter of the rectangle
    let x = rect.point().x();
    let y = rect.point().y();

    // Connect the "top side" of the rectangle
    for i in 0..rect.width() {
        let start = point(x + i, y);
        let end = point(x + i + 1, y);
        handle_points(map, rect, start, end);
    }

    // Connect the "left side" of the rectangle
    for i in 0..rect.height() {
        let start = point(x, y + i);
        let end = point(x, y + i + 1);
        handle_points(map, rect, start, end);
    }

    // Connect the "bottom side" of the rectangle
    for i in 0..rect.width() {
        let start = point(x + i, y + rect.height());
        let end = point(x + i + 1, y + rect.height());
        handle_points(map, rect, start, end);
    }

    // Connect the "right side" of the rectangle
    for i in 0..rect.height() {
        let start = point(x + rect.width(), y + i);
        let end = point(x + rect.width(), y + i + 1);
        handle_points(map, rect, start, end);
    }
}

fn handle_points(map: &mut Map, rect: &Rect, start: Point, end: Point) {
    match rect.boolean_op() {
        ShapeBoolean::Or => {
            map.connect(start, end);
        }
        ShapeBoolean::Xor => {
            if map.are_connected(start, end) {
                map.disconnect(start, end);
            } else {
                map.connect(start, end);
            }
        }
    }
}

fn point(x: usize, y: usize) -> Point {
    Point::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::super::shapes::ShapeBoolean;
    use super::*;

    #[test]
    fn test_generate_empty_map() {
        let mut ast = AbstractSyntaxTree::new();
        ast.set_grid_dimensions(4, 3);
        let map = generate_map(&ast).expect("Bad generate");
        assert_eq!(map.width(), 4);
        assert_eq!(map.height(), 3);
    }

    #[test]
    fn test_map_with_single_cell_rectangle() {
        let mut ast = AbstractSyntaxTree::new();
        ast.set_grid_dimensions(1, 1);
        let rect = Rect::new(Point::new(0, 0), 1, 1, ShapeBoolean::Or);
        ast.add_shape(ShapeNode::Rect(rect));
        let map = generate_map(&ast).expect("Bad generate");
        assert!(map.are_connected(point(0, 0), point(1, 0)));
        assert!(map.are_connected(point(0, 0), point(0, 1)));
        assert!(map.are_connected(point(0, 1), point(1, 1)));
        assert!(map.are_connected(point(1, 0), point(1, 1)));
    }

    #[test]
    fn test_map_with_single_nontrivial_rectangle() {
        let mut ast = AbstractSyntaxTree::new();
        ast.set_grid_dimensions(10, 10);
        let rect = Rect::new(Point::new(2, 1), 3, 2, ShapeBoolean::Or);
        ast.add_shape(ShapeNode::Rect(rect));
        let map = generate_map(&ast).expect("Bad generate");

        // Top
        assert!(map.are_connected(point(2, 1), point(3, 1)));
        assert!(map.are_connected(point(3, 1), point(4, 1)));
        assert!(map.are_connected(point(4, 1), point(5, 1)));

        // Top sanity-check
        assert!(!map.are_connected(point(1, 1), point(2, 1)));
        assert!(!map.are_connected(point(5, 1), point(6, 1)));

        // Left
        assert!(map.are_connected(point(2, 1), point(2, 2)));
        assert!(map.are_connected(point(2, 2), point(2, 3)));

        // Left sanity-check
        assert!(!map.are_connected(point(2, 0), point(2, 1)));
        assert!(!map.are_connected(point(2, 3), point(2, 4)));

        // Bottom
        assert!(map.are_connected(point(2, 3), point(3, 3)));
        assert!(map.are_connected(point(3, 3), point(4, 3)));
        assert!(map.are_connected(point(4, 3), point(5, 3)));

        // Bottom sanity-check
        assert!(!map.are_connected(point(1, 3), point(2, 3)));
        assert!(!map.are_connected(point(5, 3), point(6, 3)));

        // Right
        assert!(map.are_connected(point(5, 1), point(5, 2)));
        assert!(map.are_connected(point(5, 2), point(5, 3)));

        // Right sanity-check
        assert!(!map.are_connected(point(5, 0), point(5, 1)));
        assert!(!map.are_connected(point(5, 3), point(5, 4)));
    }

    #[test]
    fn test_xor_rectangles() {
        let mut ast = AbstractSyntaxTree::new();
        ast.set_grid_dimensions(10, 10);
        let rect1 = Rect::new(Point::new(2, 1), 3, 2, ShapeBoolean::Or);
        ast.add_shape(ShapeNode::Rect(rect1));
        let rect2 = Rect::new(Point::new(5, 1), 2, 2, ShapeBoolean::Xor);
        ast.add_shape(ShapeNode::Rect(rect2));
        let map = generate_map(&ast).expect("Bad generate");

        // Check that the overlapped lines are not connected because of the XOR
        assert!(!map.are_connected(point(5, 1), point(5, 2)));
        assert!(!map.are_connected(point(5, 2), point(5, 3)));
    }
}
