/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use super::ast::{AbstractSyntaxTree, EntityNode, ShapeNode};
use super::entities::EntityPosition;
use super::lexer::lex;
use super::parse_error::{GridMapperParseError, GridMapperParseErrorType, SyntaxError};
use super::points::Point;
use super::shapes::{Rect, Shape, ShapeBoolean};
use super::token::{Token, TokenType};

pub fn parse(input: &str) -> Result<AbstractSyntaxTree, GridMapperParseError> {
    let tokens = lex(input)?;
    let parser = Parser { tokens, i: 0 };
    parser.parse()
}

struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

impl Parser {
    fn parse(mut self) -> Result<AbstractSyntaxTree, GridMapperParseError> {
        let mut ast = AbstractSyntaxTree::new();

        self.parse_grid_dimensions(&mut ast)?;

        while self.next_matches_any(&[TokenType::Rect, TokenType::Entity, TokenType::Xor]) {
            let boolean_op = self.parse_boolean_op();
            if self.next_matches(TokenType::Rect) {
                let node = self.parse_rect(boolean_op)?;
                ast.add_shape(node);
            } else if self.next_matches(TokenType::Entity) {
                let node = self.parse_entity()?;
                ast.add_entity(node);
            } else {
                panic!("Unexpected token type");
            }
        }

        Ok(ast)
    }

    fn parse_grid_dimensions(
        &mut self,
        ast: &mut AbstractSyntaxTree,
    ) -> Result<(), GridMapperParseError> {
        self.accept(TokenType::Grid)?;
        let width = self.accept_number()?;
        self.accept(TokenType::Comma)?;
        let height = self.accept_number()?;
        ast.set_grid_dimensions(width, height);
        Ok(())
    }

    fn parse_boolean_op(&mut self) -> ShapeBoolean {
        if self.next_matches(TokenType::Xor) {
            self.accept(TokenType::Xor).unwrap();
            ShapeBoolean::Xor
        } else {
            ShapeBoolean::Or
        }
    }

    fn parse_rect(&mut self, boolean_op: ShapeBoolean) -> Result<ShapeNode, GridMapperParseError> {
        self.accept(TokenType::Rect)?;
        self.accept(TokenType::At)?;
        let point = self.parse_point()?;
        self.accept(TokenType::Width)?;
        let width = self.accept_number()? as usize;
        self.accept(TokenType::Height)?;
        let height = self.accept_number()? as usize;
        let rect = Rect::new(point, width, height, boolean_op);
        let rect_node = ShapeNode::Rect(rect);
        Ok(rect_node)
    }

    fn parse_point(&mut self) -> Result<Point, GridMapperParseError> {
        let x = self.accept_number()? as usize;
        self.accept(TokenType::Comma)?;
        let y = self.accept_number()? as usize;
        Ok(Point::new(x, y))
    }

    fn parse_entity(&mut self) -> Result<EntityNode, GridMapperParseError> {
        self.accept(TokenType::Entity)?;
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
            return Err(GridMapperParseError::new(
                GridMapperParseErrorType::InvalidPosition,
                tok.position.line,
                tok.position.col,
            ));
        } else {
            let tok = self.tokens.last().unwrap();
            return Err(GridMapperParseError::new(
                GridMapperParseErrorType::UnexpectedEndOfFile,
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

        Ok(EntityNode {
            shape,
            point,
            position,
        })
    }

    fn parse_shape(&mut self) -> Result<TokenType, GridMapperParseError> {
        if self.is_at_end() {
            let tok = self.tokens.last().unwrap();
            Err(GridMapperParseError::new(
                GridMapperParseErrorType::UnexpectedEndOfFile,
                tok.position.line,
                tok.position.col,
            ))
        } else if self.next_matches(TokenType::Circle) {
            Ok(self.consume()?.token_type)
        } else {
            let token = self.consume()?;
            Err(GridMapperParseError::new(
                GridMapperParseErrorType::InvalidShape,
                token.position.line,
                token.position.col,
            ))
        }
    }

    fn accept(&mut self, token_type: TokenType) -> Result<&Token, GridMapperParseError> {
        let token = self.consume()?;
        if token_type_matches(token, token_type) {
            Ok(token)
        } else {
            Err(syntax_error(token_type, token))
        }
    }

    fn accept_number(&mut self) -> Result<u32, GridMapperParseError> {
        let token = self.consume()?;
        match token.token_type {
            TokenType::Number(n) => Ok(n),
            _ => Err(syntax_error(TokenType::Number(0), token)),
        }
    }

    fn consume(&mut self) -> Result<&Token, GridMapperParseError> {
        if self.i >= self.tokens.len() {
            return Err(GridMapperParseError::new(
                GridMapperParseErrorType::UnexpectedEndOfFile,
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

fn syntax_error(expected: TokenType, token: &Token) -> GridMapperParseError {
    let actual = token.token_type;
    let err_type = GridMapperParseErrorType::SyntaxError(SyntaxError::new(expected, actual));
    GridMapperParseError::new(err_type, token.position.line, token.position.col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid_dimensions() {
        let input = "grid 5, 3";
        let ast = parse(input).expect("Bad parse");
        let grid_node = ast.grid_dimensions().unwrap();
        assert_eq!(grid_node.width(), 5);
        assert_eq!(grid_node.height(), 3);
    }

    #[test]
    fn test_syntax_error() {
        let input = "grid width 10";
        match parse(input) {
            Ok(_) => panic!("Should fail"),
            Err(err) => match err.error_type {
                GridMapperParseErrorType::SyntaxError(err) => {
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
        assert_eq!(ast.shapes().len(), 1);
        let shape = &ast.shapes()[0];
        match shape {
            ShapeNode::Rect(rect) => {
                assert_eq!(rect.point().x(), 1);
                assert_eq!(rect.point().y(), 2);
                assert_eq!(rect.width(), 3);
                assert_eq!(rect.height(), 2);
                assert!(matches!(rect.boolean_op(), ShapeBoolean::Or));
            }
        }
    }

    #[test]
    fn test_parse_circular_entity() {
        let input = "grid 10, 10\nentity circle within 5,7";
        let ast = parse(input).expect("Bad parse");
        assert_eq!(ast.entities().len(), 1);
        let entity = &ast.entities()[0];
        assert!(matches!(entity.shape, Shape::Circle(0)));
        assert_eq!(entity.point.x(), 5);
        assert_eq!(entity.point.y(), 7);
    }

    #[test]
    fn test_parse_rect_with_xor() {
        let input = "grid 10, 10\nrect at 1, 2 width 3 height 2\nxor rect at 4,2 width 2 height 2";
        let ast = parse(input).expect("Bad parse");
        assert_eq!(ast.shapes().len(), 2);
        let shape = &ast.shapes()[1];
        match shape {
            ShapeNode::Rect(rect) => {
                assert_eq!(rect.point().x(), 4);
                assert_eq!(rect.point().y(), 2);
                assert_eq!(rect.width(), 2);
                assert_eq!(rect.height(), 2);
                assert!(matches!(rect.boolean_op(), ShapeBoolean::Xor));
            }
        }
    }

    #[test]
    fn test_parse_circular_entity_at_point() {
        let input = "grid 10, 10\nentity circle at 5,6 radius 2";
        let ast = parse(input).expect("Bad parse");
        assert_eq!(ast.entities().len(), 1);
        let entity = &ast.entities()[0];
        assert!(matches!(entity.shape, Shape::Circle(2)));
        assert_eq!(entity.point.x(), 5);
        assert_eq!(entity.point.y(), 6);
    }
}
