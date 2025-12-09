// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use enumchar::EnumChar;

pub use aoc::*;

pub const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

#[derive(EnumChar, Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum Cell {
    #[char('.')]
    Empty,
    #[char('@')]
    Roll,
    #[default]
    #[char('#')]
    Wall,
}

pub type Sqrid = sqrid::sqrid_create!(136, 136, true);
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);
pub use sqrid::Dir;

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    pub fn parse(input: &str) -> Result<Grid> {
        let vecvec = aoc::parse_with_chumsky!(vecvec(".@"), input)?;
        let mut g = Grid::default();
        g.extend_from_vecvec(vecvec)?;
        Ok(g)
    }

    #[test]
    fn test() -> Result<()> {
        let input = parser::parse(EXAMPLE)?;
        assert_eq!(input.get(&Pos::FIRST), &Cell::Empty);
        Ok(())
    }
}
