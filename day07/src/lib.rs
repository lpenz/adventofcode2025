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
pub type Pos = sqrid::pos_create!(Sqrid);
pub type Grid = sqrid::grid_create!(Sqrid, Cell);
pub use sqrid::Dir;

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    pub fn all<'src>() -> impl Parser<'src, &'src str, Vec<Vec<Cell>>, extra::Err<Rich<'src, char>>>
    {
        let cell = just('.')
            .to(Cell::Empty)
            .or(just('^').to(Cell::Splitter))
            .or(just('S').to(Cell::Start));
        let line = cell.repeated().collect().then_ignore(just('\n'));
        line.repeated().collect::<Vec<Vec<Cell>>>()
    }

    pub fn parse(input: &str) -> Result<Grid> {
        let vecvec = aoc::parse_with_chumsky!(all(), input)?;
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
