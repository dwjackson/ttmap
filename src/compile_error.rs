/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use super::source_location::SourceLocation;
use super::token::TokenType;

#[derive(Debug)]
pub struct CompileError {
    pub error_type: CompileErrorType,
    pub location: SourceLocation,
}

impl CompileError {
    pub fn new(error_type: CompileErrorType, line: usize, col: usize) -> CompileError {
        CompileError {
            error_type,
            location: SourceLocation { line, col },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompileErrorType {
    InvalidCharacter,
    UnrecognizedKeyword,
    InvalidNumber,
    UnexpectedEndOfFile,
    SyntaxError(SyntaxError),
    InvalidShape,
    InvalidPosition,
    NoGridDimensions,
    OutOfBounds,
    InvalidOrientation,
}

#[derive(Debug, Clone, Copy)]
pub struct SyntaxError {
    expected: TokenType,
    actual: TokenType,
}

impl SyntaxError {
    pub fn new(expected: TokenType, actual: TokenType) -> SyntaxError {
        SyntaxError { expected, actual }
    }

    pub fn expected(&self) -> TokenType {
        self.expected
    }

    pub fn actual(&self) -> TokenType {
        self.actual
    }
}
