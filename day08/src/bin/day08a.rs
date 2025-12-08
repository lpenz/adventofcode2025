// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashMap;

use day08::*;

fn process(maxconnections: usize, input: &str) -> Result<usize> {
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
    for (connections, (_, a, b)) in distances.iter().enumerate() {
        if connections == maxconnections {
            break;
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
            }
        } else if let Some(&c) = circuits.get(a) {
            circuits.insert(*b, c);
        } else if let Some(&c) = circuits.get(b) {
            circuits.insert(*a, c);
        } else {
            circuits.insert(*a, cmax);
            circuits.insert(*b, cmax);
            cmax += 1;
        }
    }
    let circuit_sizes =
        circuits
            .into_iter()
            .fold(HashMap::<usize, usize>::default(), |mut acc, (_, i)| {
                acc.entry(i).and_modify(|v| *v += 1).or_insert(1);
                acc
            });
    let mut circuit_sizes = circuit_sizes.values().collect::<Vec<_>>();
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    Ok(circuit_sizes.into_iter().take(3).product())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(10, EXAMPLE)?, 40);
    Ok(())
}

fn main() -> Result<()> {
    do_main(|input| process(1000, input))
}
