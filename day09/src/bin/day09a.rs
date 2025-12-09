// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day09::*;

fn process(input: &str) -> Result<i64> {
    let input = parser::parse(input)?;
    Ok(input
        .iter()
        .enumerate()
        .flat_map(|(i, a)| input[i + 1..].iter().map(|b| area(a, b)))
        .max()
        .unwrap())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 50);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
