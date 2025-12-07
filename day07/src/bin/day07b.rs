// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashMap;

use sqrid::PosT;

use day07::*;

fn process(input: &str) -> Result<usize> {
    let grid = parser::parse(input)?;
    let start = Pos::iter()
        .find(|p| grid[p] == Cell::Start)
        .ok_or_eyre("start not found")?;
    let mut head = HashMap::<Pos, usize>::new();
    head.insert(start, 1);
    for y in 0..Pos::LAST.y() {
        let mut next = HashMap::<Pos, usize>::new();
        for (pos, count) in head.drain() {
            let p = (pos + Dir::S)?;
            if grid[p] == Cell::Splitter {
                if let Ok(w) = p + Dir::W {
                    next.entry(w).and_modify(|e| *e += count).or_insert(count);
                }
                if let Ok(e) = p + Dir::E {
                    next.entry(e).and_modify(|e| *e += count).or_insert(count);
                }
            } else {
                next.entry(p).and_modify(|e| *e += count).or_insert(count);
            }
        }
        head = next;
        eprintln!("y {} head {:?}", y, head);
    }
    Ok(head.into_values().sum::<usize>())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 40);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
