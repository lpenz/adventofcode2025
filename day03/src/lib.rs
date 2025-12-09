// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

pub type Jolt = u64;

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    pub fn all<'src>() -> impl Parser<'src, &'src str, Vec<Vec<Jolt>>, extra::Err<Rich<'src, char>>>
    {
        let num = digit1().map(u64::from);
        let line = num.repeated().collect().then_ignore(just('\n'));
        line.repeated().collect()
    }

    pub fn parse(input: &str) -> Result<Vec<Vec<Jolt>>> {
        aoc::parse_with_chumsky!(all(), input)
    }

    #[test]
    fn test() -> Result<()> {
        let input = parser::parse(EXAMPLE)?;
        assert_eq!(input.len(), 4);
        Ok(())
    }
}
