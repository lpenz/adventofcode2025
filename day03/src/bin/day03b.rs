// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day03::*;

const MAXDIGITS: usize = 12;

fn process(input: &str) -> Result<Jolt> {
    let batteries = parser::parse(input)?;
    Ok(batteries
        .into_iter()
        .map(|batt| {
            let mut digits = Vec::<Jolt>::default();
            let mut pos = 0;
            for d in 0..MAXDIGITS {
                let imax = batt.len() - (MAXDIGITS - 1 - d);
                let mut vmax = batt[pos];
                pos += 1;
                for (i, &v) in batt.iter().enumerate().take(imax).skip(pos) {
                    if v > vmax {
                        vmax = v;
                        pos = i + 1;
                    }
                }
                digits.push(vmax);
            }
            digits
                .into_iter()
                .enumerate()
                .map(|(i, v)| v * 10_u64.pow((MAXDIGITS - 1 - i) as u32))
                .sum::<Jolt>()
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 3121910778619);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
