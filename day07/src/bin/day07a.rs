// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashSet;

use sqrid::PosT;

use day07::*;

fn process(input: &str) -> Result<usize> {
    let grid = parser::parse(input)?;
    let start = Pos::iter()
        .find(|p| grid[p] == Cell::Start)
        .ok_or_eyre("start not found")?;
    let mut head = HashSet::<Pos>::new();
    head.insert(start);
    let mut splits = 0;
    for _ in 0..Pos::LAST.y() {
        let mut next = HashSet::<Pos>::new();
        for pos in head.drain() {
            let p = (pos + Dir::S)?;
            if grid[p] == Cell::Splitter {
                if let Ok(w) = p + Dir::W {
                    next.insert(w);
                }
                if let Ok(e) = p + Dir::E {
                    next.insert(e);
                }
                splits += 1;
            } else {
                next.insert(p);
            }
        }
        head = next;
    }
    Ok(splits)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 21);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
