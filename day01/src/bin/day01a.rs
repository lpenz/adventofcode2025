// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day01::*;

fn process(input: &str) -> Result<usize> {
    let input = parser::parse(input)?;
    let mut pos = 50_i32;
    let mut count = 0;
    for (rot, dist) in input {
        match rot {
            Rotation::Left => {
                pos -= dist;
            }
            Rotation::Right => {
                pos += dist;
            }
        }
        pos %= 100;
        if pos == 0 {
            count += 1;
        }
    }
    Ok(count)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 3);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
