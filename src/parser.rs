/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use crate::ast::GridDimensionsNode;

use super::ast::{AbstractSyntaxTree, AstNode, AstNodeType, EntityNode, ShapeNode};
use super::compile_error::{CompileError, CompileErrorType, SyntaxError};
use super::entities::EntityPosition;
use super::lexer::lex;
use super::points::Point;
use super::shapes::{Rect, Shape, ShapeBoolean};
use super::token::{Token, TokenType};

pub fn parse(input: &str) -> Result<AbstractSyntaxTree, CompileError> {
    let tokens = lex(input)?;
    let parser = Parser { tokens, i: 0 };
    parser.parse()
}

struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

impl Parser {
    fn parse(mut self) -> Result<AbstractSyntaxTree, CompileError> {
        let mut ast = AbstractSyntaxTree::new();

        let grid_dimensions_node = self.parse_grid_dimensions()?;
        ast.add_node(grid_dimensions_node);

        while self.next_matches_any(&[TokenType::Rect, TokenType::Entity, TokenType::Xor]) {
            let boolean_op = self.parse_boolean_op();
            if self.next_matches(TokenType::Rect) {
                let node = self.parse_rect(boolean_op)?;
                ast.add_node(node);
            } else if self.next_matches(TokenType::Entity) {
                let node = self.parse_entity()?;
                ast.add_node(node);
            } else {
                panic!("Unexpected token type");
            }
        }

        Ok(ast)
    }

    fn parse_grid_dimensions(&mut self) -> Result<AstNode, CompileError> {
        let position = self.accept(TokenType::Grid)?.position;
        let width = self.accept_number()?;
        self.accept(TokenType::Comma)?;
        let height = self.accept_number()?;
        let node_type = AstNodeType::GridDimensions(GridDimensionsNode::new(width, height));
        let node = AstNode::new(node_type, position);
        Ok(node)
    }

    fn parse_boolean_op(&mut self) -> ShapeBoolean {
        if self.next_matches(TokenType::Xor) {
            self.accept(TokenType::Xor).unwrap();
            ShapeBoolean::Xor
        } else {
            ShapeBoolean::Or
        }
    }

    fn parse_rect(&mut self, boolean_op: ShapeBoolean) -> Result<AstNode, CompileError> {
        let position = self.accept(TokenType::Rect)?.position;
        self.accept(TokenType::At)?;
        let point = self.parse_point()?;
        self.accept(TokenType::Width)?;
        let width = self.accept_number()? as usize;
        self.accept(TokenType::Height)?;
        let height = self.accept_number()? as usize;
        let rect = Rect::new(point, width, height, boolean_op);
        let shape_node = ShapeNode::Rect(rect);
        let node_type = AstNodeType::Shape(shape_node);
        let node = AstNode::new(node_type, position);
        Ok(node)
    }

    fn parse_point(&mut self) -> Result<Point, CompileError> {
        let x = self.accept_number()? as usize;
        self.accept(TokenType::Comma)?;
        let y = self.accept_number()? as usize;
        Ok(Point::new(x, y))
    }

    fn parse_entity(&mut self) -> Result<AstNode, CompileError> {
        let node_position = self.accept(TokenType::Entity)?.position;
        let shape_token_type = self.parse_shape()?;
        let position: EntityPosition;
        if self.next_matches(TokenType::Within) {
            self.accept(TokenType::Within)?;
            position = EntityPosition::Within;
        } else if self.next_matches(TokenType::At) {
            self.accept(TokenType::At)?;
            position = EntityPosition::At;
        } else if !self.is_at_end() {
            let tok = self.peek().unwrap();
            return Err(CompileError::new(
                CompileErrorType::InvalidPosition,
                tok.position.line,
                tok.position.col,
            ));
        } else {
            let tok = self.tokens.last().unwrap();
            return Err(CompileError::new(
                CompileErrorType::UnexpectedEndOfFile,
                tok.position.line,
                tok.position.col,
            ));
        }
        let x = self.accept_number()? as usize;
        self.accept(TokenType::Comma)?;
        let y = self.accept_number()? as usize;
        let point = Point::new(x, y);

        let shape = match shape_token_type {
            TokenType::Circle => {
                let radius = match position {
                    EntityPosition::At => {
                        self.accept(TokenType::Radius)?;
                        self.accept_number()?
                    }
                    EntityPosition::Within => 0,
                } as usize;
                Shape::Circle(radius)
            }
            _ => {
                panic!("Unexpected shape token type {:?}", shape_token_type);
            }
        };

        let node_type = AstNodeType::Entity(EntityNode {
            shape,
            point,
            position,
        });
        let node = AstNode::new(node_type, node_position);
        Ok(node)
    }

    fn parse_shape(&mut self) -> Result<TokenType, CompileError> {
        if self.is_at_end() {
            let tok = self.tokens.last().unwrap();
            Err(CompileError::new(
                CompileErrorType::UnexpectedEndOfFile,
                tok.position.line,
                tok.position.col,
            ))
        } else if self.next_matches(TokenType::Circle) {
            Ok(self.consume()?.token_type)
        } else {
            let token = self.consume()?;
            Err(CompileError::new(
                CompileErrorType::InvalidShape,
                token.position.line,
                token.position.col,
            ))
        }
    }

    fn accept(&mut self, token_type: TokenType) -> Result<&Token, CompileError> {
        let token = self.consume()?;
        if token_type_matches(token, token_type) {
            Ok(token)
        } else {
            Err(syntax_error(token_type, token))
        }
    }

    fn accept_number(&mut self) -> Result<u32, CompileError> {
        let token = self.consume()?;
        match token.token_type {
            TokenType::Number(n) => Ok(n),
            _ => Err(syntax_error(TokenType::Number(0), token)),
        }
    }

    fn consume(&mut self) -> Result<&Token, CompileError> {
        if self.i >= self.tokens.len() {
            return Err(CompileError::new(
                CompileErrorType::UnexpectedEndOfFile,
                0,
                0,
            ));
        }
        let token = &self.tokens[self.i];
        self.i += 1;
        Ok(token)
    }

    fn next_matches_any(&self, token_types: &[TokenType]) -> bool {
        token_types.iter().any(|&tt| self.next_matches(tt))
    }

    fn next_matches(&self, token_type: TokenType) -> bool {
        match self.peek() {
            Some(token) => token_type_matches(token, token_type),
            None => false,
        }
    }

    fn peek(&self) -> Option<&Token> {
        if !self.is_at_end() {
            self.tokens.get(self.i)
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.i >= self.tokens.len()
    }
}

fn token_type_matches(token: &Token, token_type: TokenType) -> bool {
    std::mem::discriminant(&token.token_type) == std::mem::discriminant(&token_type)
}

fn syntax_error(expected: TokenType, token: &Token) -> CompileError {
    let actual = token.token_type;
    let err_type = CompileErrorType::SyntaxError(SyntaxError::new(expected, actual));
    CompileError::new(err_type, token.position.line, token.position.col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid_dimensions() {
        let input = "grid 5, 3";
        let ast = parse(input).expect("Bad parse");
        let grid_node = ast.nodes().next().unwrap();
        match grid_node.node_type() {
            AstNodeType::GridDimensions(grid_node) => {
                assert_eq!(grid_node.width(), 5);
                assert_eq!(grid_node.height(), 3);
            }
            _ => panic!("No dimensions"),
        }
    }

    #[test]
    fn test_syntax_error() {
        let input = "grid width 10";
        match parse(input) {
            Ok(_) => panic!("Should fail"),
            Err(err) => match err.error_type {
                CompileErrorType::SyntaxError(err) => {
                    assert!(matches!(err.expected(), TokenType::Number(0)));
                    assert!(matches!(err.actual(), TokenType::Width));
                }
                _ => panic!("Wrong error type"),
            },
        }
    }

    #[test]
    fn test_parse_rect() {
        let input = "grid 10, 10\nrect at 1, 2 width 3 height 2";
        let ast = parse(input).expect("Bad parse");
        let rect = rect_at_index(&ast, 1);
        assert_eq!(rect.point().x(), 1);
        assert_eq!(rect.point().y(), 2);
        assert_eq!(rect.width(), 3);
        assert_eq!(rect.height(), 2);
        assert!(matches!(rect.boolean_op(), ShapeBoolean::Or));
    }

    #[test]
    fn test_parse_circular_entity() {
        let input = "grid 10, 10\nentity circle within 5,7";
        let ast = parse(input).expect("Bad parse");
        let entity = entity_at_index(&ast, 1);
        assert!(matches!(entity.shape, Shape::Circle(0)));
        assert_eq!(entity.point.x(), 5);
        assert_eq!(entity.point.y(), 7);
    }

    #[test]
    fn test_parse_rect_with_xor() {
        let input = "grid 10, 10\nrect at 1, 2 width 3 height 2\nxor rect at 4,2 width 2 height 2";
        let ast = parse(input).expect("Bad parse");
        let rect = rect_at_index(&ast, 2);
        assert_eq!(rect.point().x(), 4);
        assert_eq!(rect.point().y(), 2);
        assert_eq!(rect.width(), 2);
        assert_eq!(rect.height(), 2);
        assert!(matches!(rect.boolean_op(), ShapeBoolean::Xor));
    }

    #[test]
    fn test_parse_circular_entity_at_point() {
        let input = "grid 10, 10\nentity circle at 5,6 radius 2";
        let ast = parse(input).expect("Bad parse");
        let entity = entity_at_index(&ast, 1);
        assert!(matches!(entity.shape, Shape::Circle(2)));
        assert_eq!(entity.point.x(), 5);
        assert_eq!(entity.point.y(), 6);
    }

    fn rect_at_index(ast: &AbstractSyntaxTree, index: usize) -> &Rect {
        let mut nodes = ast.nodes();
        for _ in 0..index {
            nodes.next();
        }
        let node = nodes.next().unwrap();
        match node.node_type() {
            AstNodeType::Shape(shape_node) => match shape_node {
                ShapeNode::Rect(rect) => rect,
            },
            _ => panic!("Not a rect node: {:?}", node.node_type()),
        }
    }

    fn entity_at_index(ast: &AbstractSyntaxTree, index: usize) -> &EntityNode {
        let mut nodes = ast.nodes();
        for _ in 0..index {
            nodes.next();
        }
        let node = nodes.next().unwrap();
        match node.node_type() {
            AstNodeType::Entity(e) => &e,
            _ => panic!("Not an entity node: {:?}", node.node_type()),
        }
    }
}
