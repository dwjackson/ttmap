/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use std::env;
use ttmap::compiler::compile_svg;
use ttmap::files::read_file;

const DEFAULT_DIMENSION: usize = 10;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("USAGE: ttmap [MAP_FILE] [DIMENSION?]");
        std::process::exit(1);
    }

    let file_name = &args[1];

    let dim = if args.len() > 2 {
        args[2].parse::<usize>().expect("Invalid dimension")
    } else {
        DEFAULT_DIMENSION
    };

    let input = read_file(file_name);
    let s = compile_svg(&input, dim);
    println!("{}", s);
}
