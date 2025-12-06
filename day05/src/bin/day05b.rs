// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use range_set::range_set;

use day05::*;

fn process(input: &str) -> Result<usize> {
    let (ranges, _) = parser::parse(input)?;
    let fresh = ranges
        .into_iter()
        .fold(range_set![], |mut rangeset, (first, last)| {
            rangeset.insert_range(first..=last);
            rangeset
        });
    Ok(fresh.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 14);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
