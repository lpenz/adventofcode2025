// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day02::*;

fn duplicate(halfnum: Pid) -> Pid {
    halfnum * factor_calc(halfnum) + halfnum
}

fn process(input: &str) -> Result<Pid> {
    let ranges = parser::parse(input)?;
    let mut invalsum = 0;
    for (min, max) in ranges {
        let digits = digits_calc(min);
        let mut halfnum = min / 10_u64.pow(digits / 2);
        if digits % 2 == 1 {
            // Fix case where we start with an odd number of digits
            halfnum = factor_calc(halfnum) / 10;
        }
        loop {
            let num = duplicate(halfnum);
            if num > max {
                break;
            }
            if num >= min {
                // Sometimes we start by going back some units
                invalsum += num;
            }
            halfnum += 1;
        }
    }
    Ok(invalsum)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 1227775554);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
