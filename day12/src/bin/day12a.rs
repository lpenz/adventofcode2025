// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day12::*;

fn process(input: &str) -> Result<usize> {
    let input = parser::parse(input)?;
    Ok(input.0.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 6);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
