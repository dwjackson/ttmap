/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use super::entities::EntityPosition;
use super::points::Point;
use super::shapes::{Line, Rect, Shape};
use crate::source_position::SourcePosition;

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    nodes: Vec<AstNode>,
}

impl AbstractSyntaxTree {
    pub fn new() -> AbstractSyntaxTree {
        AbstractSyntaxTree { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: AstNode) {
        self.nodes.push(node);
    }

    pub fn nodes(&self) -> std::slice::Iter<'_, AstNode> {
        self.nodes.iter()
    }
}

#[derive(Debug)]
pub struct AstNode {
    position: SourcePosition,
    node_type: AstNodeType,
}

impl AstNode {
    pub fn new(node_type: AstNodeType, position: SourcePosition) -> AstNode {
        AstNode {
            position,
            node_type,
        }
    }

    pub fn node_type(&self) -> &AstNodeType {
        &self.node_type
    }

    pub fn position(&self) -> SourcePosition {
        self.position
    }
}

#[derive(Debug)]
pub enum AstNodeType {
    GridDimensions(GridDimensionsNode),
    Shape(ShapeNode),
    Entity(EntityNode),
}

#[derive(Debug)]
pub struct GridDimensionsNode {
    width: usize,
    height: usize,
}

impl GridDimensionsNode {
    pub fn new(width: u32, height: u32) -> GridDimensionsNode {
        GridDimensionsNode {
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[derive(Debug)]
pub enum ShapeNode {
    Rect(Rect),
    Line(Line),
}

#[derive(Debug)]
pub struct EntityNode {
    pub shape: Shape,
    pub point: Point,
    pub position: EntityPosition,
}
