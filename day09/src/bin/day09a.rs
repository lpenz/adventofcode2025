// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day09::*;

pub fn area(a: &Pos, b: &Pos) -> i64 {
    let dx = 1 + if a.x > b.x { a.x - b.x } else { b.x - a.x };
    let dy = 1 + if a.y > b.y { a.y - b.y } else { b.y - a.y };
    dx * dy
}

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
