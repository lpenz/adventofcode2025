// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day03::*;

fn process(input: &str) -> Result<Jolt> {
    let batteries = parser::parse(input)?;
    Ok(batteries
        .iter()
        .map(|batt| {
            (0..batt.len())
                .flat_map(|i| ((i + 1)..batt.len()).map(move |j| batt[i] * 10 + batt[j]))
                .max()
                .unwrap()
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 357);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
