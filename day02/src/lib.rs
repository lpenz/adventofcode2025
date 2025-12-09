// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n";

pub type Pid = u64;

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    pub fn all<'src>() -> impl Parser<'src, &'src str, Vec<(Pid, Pid)>, extra::Err<Rich<'src, char>>>
    {
        let range = number().then_ignore(just('-')).then(number());
        let ranges = range.separated_by(just(',')).collect();
        ranges.then_ignore(just('\n'))
    }

    pub fn parse(input: &str) -> Result<Vec<(Pid, Pid)>> {
        aoc::parse_with_chumsky!(all(), input)
    }

    #[test]
    fn test() -> Result<()> {
        let input = parser::parse(EXAMPLE)?;
        assert_eq!(input.len(), 11);
        Ok(())
    }
}

pub fn digits_calc(num: Pid) -> u32 {
    format!("{}", num).len() as u32
}

pub fn factor_calc(num: Pid) -> Pid {
    let digits = digits_calc(num);
    10_u64.pow(digits)
}
