/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

extern crate getopts;
use getopts::Options;
use std::env;
use std::io;
use std::io::Read;
use ttmap::compiler::compile_svg;
use ttmap::files::read_file;

const DEFAULT_DIMENSION: usize = 10;

// Options
const OPT_FILE: &str = "f";
const OPT_DIMENSION: &str = "d";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt(OPT_FILE, "file", "input map file", "MAP_FILE");
    opts.optopt(
        OPT_DIMENSION,
        "dimension",
        "map cell dimension in pixels",
        "DIMENSION",
    );
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("{}", e.to_string()),
    };

    let input = if matches.opt_present(OPT_FILE) {
        let file_name = matches.opt_str(OPT_FILE).unwrap();
        read_file(&file_name)
    } else {
        let mut s = String::new();
        io::stdin()
            .read_to_string(&mut s)
            .expect("Could not read stdin");
        s
    };

    let dim = if matches.opt_present(OPT_DIMENSION) {
        matches
            .opt_str(OPT_DIMENSION)
            .unwrap()
            .parse::<usize>()
            .expect("Invalid dimension")
    } else {
        DEFAULT_DIMENSION
    };

    let s = compile_svg(&input, dim);
    println!("{}", s);
}
