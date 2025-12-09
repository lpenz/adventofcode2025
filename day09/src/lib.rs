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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    pub fn all<'src>() -> impl Parser<'src, &'src str, Vec<Pos>, extra::Err<Rich<'src, char>>> {
        let line = number()
            .then_ignore(just(','))
            .then(number())
            .map(|(x, y)| Pos { x, y })
            .then_ignore(just('\n'));
        line.repeated().collect()
    }

    pub fn parse(input: &str) -> Result<Vec<Pos>> {
        chumsky_parse(input, all())
    }

    #[test]
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(input.len(), 8);
        Ok(())
    }
}

pub fn area(a: &Pos, b: &Pos) -> i64 {
    let dx = 1 + if a.x > b.x { a.x - b.x } else { b.x - a.x };
    let dy = 1 + if a.y > b.y { a.y - b.y } else { b.y - a.y };
    dx * dy
}
