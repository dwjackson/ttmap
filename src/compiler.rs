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
    match err.error_type {
        CompileErrorType::SyntaxError(e) => {
            format!(
                "Syntax Error: Expected {:?}, got {:?}",
                e.expected(),
                e.actual()
            )
        }
        _ => format!("{:?}", err),
    }
}
