/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

mod ast;
mod compile_error;
pub mod compiler;
mod entities;
pub mod files;
mod generator;
mod graph;
mod lexer;
pub mod map;
mod parser;
mod points;
mod shapes;
mod source_location;
mod svg;
mod token;
