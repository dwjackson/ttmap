/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use std::path::Path;
use ttmap::compiler::compile_svg;
use ttmap::files::read_file;

const DIMENSION: usize = 10;
const TESTS_DIR: &str = "tests";
const MAPS_DIR: &str = "maps";
const SVGS_DIR: &str = "svgs";

#[test]
fn test_basic_rectangle() {
    run_test("basic_test");
}

#[test]
fn test_right_angle_path() {
    run_test("right_angle_path_test");
}

#[test]
fn test_basic_circle_entity() {
    run_test("circle_entity_test");
}

#[test]
fn test_large_circle_entity() {
    run_test("large_circle_test");
}

#[test]
fn test_xor_rect() {
    run_test("xor_rect_test");
}

#[test]
fn test_xor_line() {
    run_test("xor_line_test");
}

fn run_test(test_name: &str) {
    let tests_path = Path::new(TESTS_DIR);

    let map_file_name = format!("{}.map", test_name);
    let maps_path = tests_path.join(Path::new(MAPS_DIR));
    let map_path = maps_path.join(Path::new(&map_file_name));

    let svg_file_name = format!("{}.svg", test_name);
    let svgs_path = tests_path.join(Path::new(SVGS_DIR));
    let svg_path = svgs_path.join(Path::new(&svg_file_name));
    let expected_svg = read_file(svg_path.to_str().unwrap());

    let input = read_file(map_path.to_str().unwrap());
    let svg = compile_svg(&input, DIMENSION);
    assert_eq!(svg.trim(), expected_svg.trim());
}
