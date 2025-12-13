// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use enumchar::EnumChar;

pub use aoc::*;

pub const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

#[derive(EnumChar, Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum Cell {
    #[default]
    #[char('.')]
    Empty,
    #[char('#')]
    Filled,
}

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    type Shape = Vec<Vec<Cell>>;
    type Region = (usize, usize, Vec<usize>);

    pub fn parse(input: &str) -> Result<(Vec<Shape>, Vec<Region>)> {
        let shape = number::<usize>()
            .ignore_then(just(":\n"))
            .ignore_then(vecvec::<Cell>(".#"));
        let fits = just(" ").ignore_then(number());
        let regions = number()
            .then_ignore(just("x"))
            .then(number())
            .then_ignore(just(":"))
            .then(fits.repeated().collect())
            .then_ignore(just("\n"))
            .map(|((a, b), c)| (a, b, c));
        chumsky_parse(
            input,
            shape
                .repeated()
                .collect::<Vec<Shape>>()
                .then(regions.repeated().collect()),
        )
    }

    #[test]
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(input.0.len(), 6);
        assert_eq!(input.1.len(), 3);
        Ok(())
    }
}
