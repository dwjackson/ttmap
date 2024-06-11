/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use crate::generator::{generate_map, GridMapperGenerateError};
use crate::map::map_to_svg;
use crate::parse_error::{GridMapperParseError, GridMapperParseErrorType};
use crate::parser::parse;

pub fn compile_svg(input: &str, dim: usize) -> String {
    match parse(input) {
        Ok(ast) => match generate_map(&ast) {
            Ok(map) => map_to_svg(&map, dim),
            Err(e) => format_generate_error(e),
        },
        Err(e) => format_parse_error(e),
    }
}

fn format_parse_error(err: GridMapperParseError) -> String {
    match err.error_type {
        GridMapperParseErrorType::SyntaxError(e) => {
            format!(
                "Syntax Error: Expected {:?}, got {:?}",
                e.expected(),
                e.actual()
            )
        }
        _ => format!("{:?}", err),
    }
}

fn format_generate_error(err: GridMapperGenerateError) -> String {
    format!("{:?}", err)
}
