/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_name: &str) -> String {
    let mut file = match File::open(file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name, why),
        Ok(f) => f,
    };

    let mut content = String::new();
    if let Err(why) = file.read_to_string(&mut content) {
        panic!("Couldn't read {}: {}", file_name, why);
    }

    content
}
