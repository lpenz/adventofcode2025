// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

#[derive(enumchar::EnumChar, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Cell {
    #[default]
    #[char('.')]
    Empty,
    #[char('^')]
    Splitter,
    #[char('S')]
    Start,
}

pub type Sqrid = sqrid::sqrid_create!(140, 141, true);
// pub type Sqrid = sqrid::sqrid_create!(14, 15, true);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);
pub use sqrid::Dir;

pub mod parser {
    use aoc::parser_chumsky::*;

    use super::*;

    pub fn parse(input: &str) -> Result<Grid> {
        let vecvec = chumsky_parse(input, vecvec(".^S"))?;
        let mut g = Grid::default();
        g.extend_from_vecvec(vecvec)?;
        Ok(g)
    }

    #[test]
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(
            input.iter().filter(|&cell| cell == &Cell::Splitter).count(),
            22
        );
        Ok(())
    }
}
