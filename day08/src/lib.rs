// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub fn distance2(a: &Pos, b: &Pos) -> i64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

pub const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    pub fn all<'src>() -> impl Parser<'src, &'src str, Vec<Pos>, extra::Err<Rich<'src, char>>> {
        let line = number()
            .then_ignore(just(','))
            .then(number())
            .then_ignore(just(','))
            .then(number())
            .map(|((x, y), z)| Pos { x, y, z })
            .then_ignore(just('\n'));
        line.repeated().collect()
    }

    pub fn parse(input: &str) -> Result<Vec<Pos>> {
        aoc::parse_with_chumsky!(all(), input)
    }

    #[test]
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(input.len(), 20);
        Ok(())
    }
}
