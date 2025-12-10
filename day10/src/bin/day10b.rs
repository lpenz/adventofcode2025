// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::ops::Add;

use z3::Optimize;
use z3::ast::Int;

use day10::*;

fn process(input: &str) -> Result<u64> {
    let machines = parser::parse(input)?;
    Ok(machines
        .into_iter()
        .map(|(_, buttons, target)| {
            let solver = Optimize::new();
            let numbuttons = buttons.len();
            // Variables to solve: the time each button was pressed
            let presses = (0..numbuttons)
                .map(|i| Int::new_const(format!("presses{}", i)))
                .collect::<Vec<_>>();
            // Make sure button presses are all positive
            for p in &presses {
                solver.assert(&p.ge(0))
            }
            // The master equation:
            for (it, t) in target.0.iter().enumerate() {
                let mut line = Int::from_u64(0);
                for (ib, b) in buttons.iter().enumerate() {
                    if b.0.contains(&it) {
                        line = line.add(&presses[ib]);
                    }
                }
                solver.assert(&line.eq(Int::from_u64(*t as u64)));
            }
            // Put the total number of button presses in the solver too
            let sol = Int::new_const("solution");
            let mut solformula = Int::from_u64(0);
            for p in &presses {
                solformula = solformula.add(p);
            }
            solver.assert(&sol.eq(solformula));
            // ...so that we minimize it
            solver.minimize(&sol);
            solver.check(&[]);
            let model = solver.get_model().unwrap();
            model.eval(&sol, true).and_then(|v| v.as_u64()).unwrap()
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 33);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
