// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day06::*;

fn process(input: &str) -> Result<i64> {
    let (ops, nums) = parser::parse(input)?;
    Ok(ops
        .into_iter()
        .enumerate()
        .map(|(col, op)| match op {
            '+' => nums.iter().map(|line| line[col]).sum::<i64>(),
            '*' => nums.iter().map(|line| line[col]).product(),
            _ => panic!("unknown operation {}", op),
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 4277556);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
