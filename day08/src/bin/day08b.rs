// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashMap;

use day08::*;

fn process(input: &str) -> Result<i64> {
    let posvec = parser::parse(input)?;
    let mut distances = posvec
        .iter()
        .flat_map(|a| {
            posvec
                .iter()
                .filter(move |b| *b > a)
                .map(|b| (distance2(a, b), *a, *b))
        })
        .collect::<Vec<(i64, Pos, Pos)>>();
    distances.sort();
    let mut circuits = HashMap::<Pos, usize>::default();
    let mut cmax = 0;
    let mut last2 = vec![0, 0];
    for (_, a, b) in distances.iter() {
        if circuits.len() >= posvec.len() {
            let first = circuits.values().next().unwrap();
            if circuits.values().all(|v| v == first) {
                break;
            }
        }
        if let Some(&ca) = circuits.get(a)
            && let Some(&cb) = circuits.get(b)
        {
            if ca != cb {
                // move all cb to ca
                for (_, circuit) in circuits.iter_mut() {
                    if *circuit == cb {
                        *circuit = ca;
                    }
                }
                last2 = vec![a.x, b.x];
            }
        } else if let Some(&c) = circuits.get(a) {
            circuits.insert(*b, c);
            last2 = vec![a.x, b.x];
        } else if let Some(&c) = circuits.get(b) {
            circuits.insert(*a, c);
            last2 = vec![a.x, b.x];
        } else {
            circuits.insert(*a, cmax);
            circuits.insert(*b, cmax);
            cmax += 1;
            last2 = vec![a.x, b.x];
        }
    }
    Ok(last2.into_iter().product())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 25272);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
