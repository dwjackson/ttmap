/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use crate::ast::AstNodeType;

use crate::ast::{AbstractSyntaxTree, EntityNode, GridDimensionsNode, ShapeNode};
use crate::compile_error::{CompileError, CompileErrorType};
use crate::entities::Entity;
use crate::map::Map;
use crate::points::Point;
use crate::shapes::{Line, LineOrientation, Rect, Shape, ShapeBoolean};
use crate::source_location::SourceLocation;

pub fn generate_map(ast: &AbstractSyntaxTree) -> Result<Map, CompileError> {
    let dims = find_grid_dimensions(ast);
    if dims.is_none() {
        return Err(CompileError::new(CompileErrorType::NoGridDimensions, 1, 1));
    }
    let dims = dims.unwrap();

    let mut map = Map::new(dims.width(), dims.height());

    for ast_node in ast.nodes() {
        match ast_node.node_type() {
            AstNodeType::GridDimensions(_) => (),
            AstNodeType::Shape(shape_node) => match shape_node {
                ShapeNode::Rect(rect) => {
                    handle_rect(&mut map, rect, ast_node.location())?;
                }
                ShapeNode::Line(line) => {
                    handle_line(&mut map, line, ast_node.location())?;
                }
            },
            AstNodeType::Entity(entity_node) => {
                handle_entity(&mut map, entity_node, ast_node.location())?;
            }
        }
    }

    Ok(map)
}

fn find_grid_dimensions(ast: &AbstractSyntaxTree) -> Option<&GridDimensionsNode> {
    let node = ast
        .nodes()
        .find(|n| matches!(n.node_type(), AstNodeType::GridDimensions(_)));
    node?;
    let node = node.unwrap();
    if let AstNodeType::GridDimensions(g) = node.node_type() {
        Some(g)
    } else {
        None
    }
}

fn handle_rect(map: &mut Map, rect: &Rect, location: SourceLocation) -> Result<(), CompileError> {
    // Connect all the points on the perimiter of the rectangle
    let x = rect.point().x();
    let y = rect.point().y();

    // Connect the "top side" of the rectangle
    for i in 0..rect.width() {
        let start = point(x + i, y);
        let end = point(x + i + 1, y);
        handle_rect_points(map, rect, start, end, location)?;
    }

    // Connect the "left side" of the rectangle
    for i in 0..rect.height() {
        let start = point(x, y + i);
        let end = point(x, y + i + 1);
        handle_rect_points(map, rect, start, end, location)?;
    }

    // Connect the "bottom side" of the rectangle
    for i in 0..rect.width() {
        let start = point(x + i, y + rect.height());
        let end = point(x + i + 1, y + rect.height());
        handle_rect_points(map, rect, start, end, location)?;
    }

    // Connect the "right side" of the rectangle
    for i in 0..rect.height() {
        let start = point(x + rect.width(), y + i);
        let end = point(x + rect.width(), y + i + 1);
        handle_rect_points(map, rect, start, end, location)?;
    }

    Ok(())
}

fn handle_line(map: &mut Map, line: &Line, location: SourceLocation) -> Result<(), CompileError> {
    let start = match line.orientation() {
        LineOrientation::Left | LineOrientation::Top => line.start(),
        LineOrientation::Right => line.start().right(),
        LineOrientation::Bottom => line.start().down(),
    };

    let mut p = start;
    for _ in 0..line.length() {
        let p2 = match line.orientation() {
            LineOrientation::Left | LineOrientation::Right => p.down(),
            LineOrientation::Top | LineOrientation::Bottom => p.right(),
        };

        if !(map.point_exists(p) && map.point_exists(p2)) {
            return Err(CompileError::new(
                CompileErrorType::OutOfBounds,
                location.line,
                location.col,
            ));
        }

        if matches!(line.boolean_op(), ShapeBoolean::Xor) && map.are_connected(p, p2) {
            map.disconnect(p, p2);
        } else {
            map.connect(p, p2);
        }
        p = p2;
    }

    Ok(())
}

fn handle_entity(
    map: &mut Map,
    entity_node: &EntityNode,
    location: SourceLocation,
) -> Result<(), CompileError> {
    match entity_node.shape {
        Shape::Circle(r) => {
            // Check for out-of-bounds
            let center = entity_node.point;
            if r > center.x() {
                return Err(out_of_bounds(location));
            }
            let left = Point::new(center.x() - r, center.y());
            if r > center.y() {
                return Err(out_of_bounds(location));
            }
            let top = Point::new(center.x(), center.y() - r);
            let right = Point::new(center.x() + r, center.y());
            let bottom = Point::new(center.x() + r, center.y());
            let points = [center, left, top, right, bottom];
            if points.iter().any(|p| !map.point_exists(*p)) {
                return Err(out_of_bounds(location));
            }
        }
        Shape::Square | Shape::Stair | Shape::Ladder | Shape::X => (),
    }
    let entity = Entity::new(entity_node.shape, entity_node.point, entity_node.position);
    map.add_entity(entity);
    Ok(())
}

fn out_of_bounds(location: SourceLocation) -> CompileError {
    CompileError::new(CompileErrorType::OutOfBounds, location.line, location.col)
}

fn handle_rect_points(
    map: &mut Map,
    rect: &Rect,
    start: Point,
    end: Point,
    location: SourceLocation,
) -> Result<(), CompileError> {
    if !map.point_exists(start) || !map.point_exists(end) {
        return Err(CompileError::new(
            CompileErrorType::OutOfBounds,
            location.line,
            location.col,
        ));
    }

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
    Ok(())
}

fn point(x: usize, y: usize) -> Point {
    Point::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AstNode, EntityNode};
    use crate::entities::EntityPosition;
    use crate::shapes::{LineOrientation, Shape, ShapeBoolean};

    #[test]
    fn test_generate_empty_map() {
        let mut ast = AbstractSyntaxTree::new();
        ast.add_node(dimensions(4, 3));
        let map = generate_map(&ast).expect("Bad generate");
        assert_eq!(map.width(), 4);
        assert_eq!(map.height(), 3);
    }

    #[test]
    fn test_map_with_single_cell_rectangle() {
        let mut ast = AbstractSyntaxTree::new();
        ast.add_node(dimensions(1, 1));
        let rect = Rect::new(Point::new(0, 0), 1, 1, ShapeBoolean::Or);
        ast.add_node(rect_node(rect));
        let map = generate_map(&ast).expect("Bad generate");
        assert!(map.are_connected(point(0, 0), point(1, 0)));
        assert!(map.are_connected(point(0, 0), point(0, 1)));
        assert!(map.are_connected(point(0, 1), point(1, 1)));
        assert!(map.are_connected(point(1, 0), point(1, 1)));
    }

    #[test]
    fn test_map_with_single_nontrivial_rectangle() {
        let mut ast = AbstractSyntaxTree::new();
        ast.add_node(dimensions(10, 10));
        let rect = Rect::new(Point::new(2, 1), 3, 2, ShapeBoolean::Or);
        ast.add_node(rect_node(rect));
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
        ast.add_node(dimensions(10, 10));
        let rect1 = Rect::new(Point::new(2, 1), 3, 2, ShapeBoolean::Or);
        ast.add_node(rect_node(rect1));
        let rect2 = Rect::new(Point::new(5, 1), 2, 2, ShapeBoolean::Xor);
        ast.add_node(rect_node(rect2));
        let map = generate_map(&ast).expect("Bad generate");

        // Check that the overlapped lines are not connected because of the XOR
        assert!(!map.are_connected(point(5, 1), point(5, 2)));
        assert!(!map.are_connected(point(5, 2), point(5, 3)));
    }

    #[test]
    fn test_rect_out_of_bounds() {
        let mut ast = AbstractSyntaxTree::new();
        ast.add_node(dimensions(5, 5));
        let rect = Rect::new(Point::new(2, 2), 10, 10, ShapeBoolean::Or);
        ast.add_node(rect_node(rect));
        match generate_map(&ast) {
            Ok(_) => panic!("Should fail"),
            Err(e) => {
                assert!(matches!(e.error_type, CompileErrorType::OutOfBounds));
            }
        }
    }

    #[test]
    fn test_entity_out_of_bounds() {
        let mut ast = AbstractSyntaxTree::new();
        ast.add_node(dimensions(5, 5));
        ast.add_node(circle_entity(Point::new(4, 3), 4));
        match generate_map(&ast) {
            Ok(_) => panic!("Should fail"),
            Err(e) => {
                assert!(matches!(e.error_type, CompileErrorType::OutOfBounds));
            }
        }
    }

    #[test]
    fn test_vertical_line() {
        let mut ast = AbstractSyntaxTree::new();
        ast.add_node(dimensions(10, 10));
        let line = Line::new(LineOrientation::Left, Point::new(1, 2), 4, ShapeBoolean::Or);
        let shape_node = ShapeNode::Line(line);
        let location = SourceLocation { line: 1, col: 1 };
        let ast_node = AstNode::new(AstNodeType::Shape(shape_node), location);
        ast.add_node(ast_node);
        let map = generate_map(&ast).expect("Bad generate");
        assert!(map.are_connected(Point::new(1, 3), Point::new(1, 4)));
    }

    #[test]
    fn test_horizontal_line() {
        let mut ast = AbstractSyntaxTree::new();
        ast.add_node(dimensions(10, 10));
        let line = Line::new(
            LineOrientation::Bottom,
            Point::new(2, 2),
            3,
            ShapeBoolean::Or,
        );
        let shape_node = ShapeNode::Line(line);
        let location = SourceLocation { line: 1, col: 1 };
        let ast_node = AstNode::new(AstNodeType::Shape(shape_node), location);
        ast.add_node(ast_node);
        let map = generate_map(&ast).expect("Bad generate");
        assert!(map.are_connected(Point::new(3, 3), Point::new(4, 3)));
    }

    fn dimensions(width: u32, height: u32) -> AstNode {
        let grid_dimensions_node = GridDimensionsNode::new(width, height);
        let node_type = AstNodeType::GridDimensions(grid_dimensions_node);
        let location = SourceLocation { line: 1, col: 1 };
        AstNode::new(node_type, location)
    }

    fn rect_node(rect: Rect) -> AstNode {
        let shape_node = ShapeNode::Rect(rect);
        let node_type = AstNodeType::Shape(shape_node);
        let location = SourceLocation { line: 1, col: 1 };
        AstNode::new(node_type, location)
    }

    fn circle_entity(point: Point, radius: usize) -> AstNode {
        let entity_node = EntityNode {
            shape: Shape::Circle(radius),
            point,
            position: EntityPosition::At,
        };
        let node_type = AstNodeType::Entity(entity_node);
        let location = SourceLocation { line: 1, col: 1 };
        AstNode::new(node_type, location)
    }
}
