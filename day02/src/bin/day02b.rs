// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::BTreeSet;

use day02::*;

fn uplicate(num: Pid, mult: u32) -> Pid {
    let factor = factor_calc(num);
    let mut ret = num;
    for _ in 1..mult {
        ret = ret * factor + num;
    }
    ret
}

fn process(input: &str) -> Result<Pid> {
    let ranges = parser::parse(input)?;
    let mut invalsum = 0;
    for (min, max) in ranges {
        let mut found = BTreeSet::<Pid>::default();
        let maxdigits = digits_calc(max);
        for digits in 1..=maxdigits / 2 {
            for mult in 2..=maxdigits / digits {
                let mut num = 1;
                loop {
                    let fullnum = uplicate(num, mult);
                    if digits_calc(num) > digits || fullnum > max {
                        break;
                    }
                    if fullnum >= min && !found.contains(&fullnum) {
                        // Sometimes we start by going back some units
                        invalsum += fullnum;
                        found.insert(fullnum);
                    }
                    num += 1;
                }
            }
        }
    }
    Ok(invalsum)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 4174379265);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
