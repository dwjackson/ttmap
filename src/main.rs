/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use std::env;
use std::fs::File;
use std::io::prelude::*;
use ttmap::compiler::compile_svg;

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

    let mut file = match File::open(file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name, why),
        Ok(f) => f,
    };

    let mut input = String::new();
    if let Err(why) = file.read_to_string(&mut input) {
        panic!("Couldn't read {}: {}", file_name, why);
    }

    let s = compile_svg(&input, dim);
    println!("{}", s);
}
