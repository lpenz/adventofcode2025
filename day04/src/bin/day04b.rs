// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashMap;
use std::collections::HashSet;

use day04::*;

fn process(input: &str) -> Result<usize> {
    let grid = parser::parse(input)?;
    let mut rolls = grid
        .iter_pos()
        .filter_map(|(pos, cell)| (*cell == Cell::Roll).then_some(pos))
        .collect::<HashSet<Pos>>();
    let start_total = rolls.len();
    // Calculate neighbors
    let mut neighbors = HashMap::<Pos, usize>::default();
    for pos in rolls.iter() {
        for dir in Dir::iter::<true>() {
            if let Ok(p) = pos + dir
                && rolls.contains(&p)
            {
                neighbors.entry(p).and_modify(|c| *c += 1).or_insert(1);
            }
        }
    }
    // Remove removable rolls, one by one
    loop {
        let Some(&remroll) = rolls
            .iter()
            .find(|pos| neighbors.get(pos).is_none_or(|n| *n < 4))
        else {
            break;
        };
        for dir in Dir::iter::<true>() {
            if let Ok(p) = remroll + dir
                && let Some(e) = neighbors.get_mut(&p)
            {
                if *e == 1 {
                    neighbors.remove(&p);
                } else {
                    *e -= 1;
                }
            }
        }
        rolls.remove(&remroll);
    }
    Ok(start_total - rolls.len())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 43);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
