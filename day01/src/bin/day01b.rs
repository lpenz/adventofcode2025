// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day01::*;

fn process(input: &str) -> Result<i32> {
    let input = parser::parse(input)?;
    let mut pos = 50_i32;
    let mut count = 0_i32;
    for (rot, dist) in input {
        let pre = pos;
        match rot {
            Rotation::Left => {
                pos -= dist;
            }
            Rotation::Right => {
                pos += dist;
            }
        }
        if pre > 0 && pos < 0 {
            count += 1;
        }
        if pos < 0 {
            count += (-pos - 1) / 100;
        } else {
            count += (pos - 1) / 100;
        }
        if pos < 0 {
            pos += 100 + 100 * (-pos / 100);
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
    assert_eq!(process(EXAMPLE)?, 6);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
