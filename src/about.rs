// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

pub fn print() {
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");

    println!();
    println!("  Arborist CLI {version}");
    println!("  {description}");
    println!();
    println!("  Author:  Strange Days Tech, S.A.S.");
    println!("  License: MIT OR Apache-2.0");
    println!("  Repo:    https://github.com/StrangeDaysTech/arborist-cli");
    println!("  Web:     https://strangedays.tech");
    println!();
}