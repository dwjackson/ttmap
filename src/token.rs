/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use super::source_position::SourcePosition;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub position: SourcePosition,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, col: usize) -> Token {
        Token {
            token_type,
            position: SourcePosition { line, col },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Grid,
    At,
    Width,
    Height,
    Entity,
    Rect,
    Circle,
    Within,
    Number(u32),
    Comma,
    Xor,
    Radius,
    Line,
    Along,
    From,
    Left,
    Right,
    Top,
    Bottom,
    Length,
    Square,
}
