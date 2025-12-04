// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashMap;

use day04::*;

fn process(input: &str) -> Result<usize> {
    let grid = parser::parse(input)?;
    let mut neighbors = HashMap::<Pos, usize>::default();
    for (pos, &cell) in grid.iter_pos() {
        if cell != Cell::Roll {
            continue;
        }
        for dir in Dir::iter::<true>() {
            if let Ok(p) = pos + dir
                && grid[p] == Cell::Roll
            {
                neighbors.entry(p).and_modify(|c| *c += 1).or_insert(1);
            }
        }
    }
    Ok(grid
        .iter_pos()
        .filter(|(pos, cell)| cell == &&Cell::Roll && neighbors.get(pos).is_none_or(|&n| n < 4))
        .count())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 13);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
