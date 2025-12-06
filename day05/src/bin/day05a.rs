// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day05::*;

fn process(input: &str) -> Result<usize> {
    let (ranges, ingredients) = parser::parse(input)?;
    Ok(ingredients
        .into_iter()
        .filter(|id| ranges.iter().any(|(first, last)| first <= id && id <= last))
        .count())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 3);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
