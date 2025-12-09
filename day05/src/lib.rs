// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

pub type Num = i64;
pub type Range = (Num, Num);

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    pub fn all<'src>()
    -> impl Parser<'src, &'src str, (Vec<Range>, Vec<Num>), extra::Err<Rich<'src, char>>> {
        let num = text::int(10).from_str().unwrapped();
        let numline = num.then_ignore(just('\n'));
        let rangeline = num.then_ignore(just('-')).then(num).then_ignore(just('\n'));
        rangeline
            .repeated()
            .collect()
            .then_ignore(just('\n'))
            .then(numline.repeated().collect())
    }

    pub fn parse(input: &str) -> Result<(Vec<Range>, Vec<Num>)> {
        aoc::parse_with_chumsky!(all(), input)
    }

    #[test]
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(input.0.len(), 4);
        assert_eq!(input.1.len(), 6);
        Ok(())
    }
}
