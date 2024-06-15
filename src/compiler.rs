/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use crate::compile_error::{CompileError, CompileErrorType};
use crate::generator::generate_map;
use crate::map::map_to_svg;
use crate::parser::parse;

pub fn compile_svg(input: &str, dim: usize) -> String {
    match parse(input) {
        Ok(ast) => match generate_map(&ast) {
            Ok(map) => map_to_svg(&map, dim),
            Err(e) => format_compile_error(e),
        },
        Err(e) => format_compile_error(e),
    }
}

fn format_compile_error(err: CompileError) -> String {
    let message = match err.error_type {
        CompileErrorType::SyntaxError(e) => {
            &format!("Expected {:?}, got {:?}", e.expected(), e.actual())
        }
        CompileErrorType::InvalidCharacter => "Invalid character",
        CompileErrorType::UnrecognizedKeyword => "Unrecognized keyword",
        CompileErrorType::InvalidNumber => "Invalid number",
        CompileErrorType::UnexpectedEndOfFile => "Unexpected end-of-file",
        CompileErrorType::InvalidShape => "Invalid shape",
        CompileErrorType::InvalidPosition => "Invalid position",
        CompileErrorType::NoGridDimensions => "No grid dimensions",
        CompileErrorType::OutOfBounds => "Out-of-bounds point",
        CompileErrorType::InvalidOrientation => "Invalid orientation",
    };
    format!(
        "[{},{}] ERROR: {}",
        err.position.line, err.position.col, message
    )
}
