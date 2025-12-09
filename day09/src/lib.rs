// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

#[derive(Debug)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    pub fn all<'src>() -> impl Parser<'src, &'src str, Vec<Pos>, extra::Err<Rich<'src, char>>> {
        let num = text::int(10).from_str().unwrapped();
        let line = num
            .then_ignore(just(','))
            .then(num)
            .map(|(x, y)| Pos { x, y })
            .then_ignore(just('\n'));
        line.repeated().collect()
    }

    pub fn parse(input: &str) -> Result<Vec<Pos>> {
        aoc::parse_with_chumsky!(all(), input)
    }

    #[test]
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(input.len(), 8);
        Ok(())
    }
}
