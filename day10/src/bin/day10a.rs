// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashSet;

use day10::*;

fn process(input: &str) -> Result<usize> {
    let machines = parser::parse(input)?;
    Ok(machines
        .into_iter()
        .map(|(target, buttons, _)| {
            let alloff = Lights::new(target.len());
            if target == alloff {
                return 0;
            }
            let mut current = HashSet::new();
            current.insert(alloff);
            let mut presses = 1;
            loop {
                let mut next = HashSet::new();
                for button in &buttons {
                    for c in &current {
                        let newlights = c.new_pressed(button);
                        if newlights == target {
                            return presses;
                        }
                        next.insert(newlights);
                    }
                }
                presses += 1;
                current = next;
            }
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 7);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
