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
use super::shapes::{Rect, Shape};

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    grid_dimensions_node: Option<GridDimensionsNode>,
    shapes: Vec<ShapeNode>,
    entities: Vec<EntityNode>,
}

impl AbstractSyntaxTree {
    pub fn new() -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            grid_dimensions_node: None,
            shapes: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn set_grid_dimensions(&mut self, width: u32, height: u32) {
        let node = GridDimensionsNode::new(width, height);
        self.grid_dimensions_node = Some(node);
    }

    pub fn grid_dimensions(&self) -> Option<&GridDimensionsNode> {
        self.grid_dimensions_node.as_ref()
    }

    pub fn shapes(&self) -> &Vec<ShapeNode> {
        &self.shapes
    }

    pub fn add_shape(&mut self, node: ShapeNode) {
        self.shapes.push(node);
    }

    pub fn entities(&self) -> &Vec<EntityNode> {
        &self.entities
    }

    pub fn add_entity(&mut self, node: EntityNode) {
        self.entities.push(node);
    }
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
}

#[derive(Debug)]
pub struct EntityNode {
    pub shape: Shape,
    pub point: Point,
    pub position: EntityPosition,
}
