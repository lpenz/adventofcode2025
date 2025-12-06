// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

use enumchar::EnumChar;

pub const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

#[derive(EnumChar, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rotation {
    #[char('L')]
    Left,
    #[char('R')]
    Right,
}

pub type Distance = i32;
pub type Position = i32;

pub mod parser {
    use chumsky::prelude::*;

    use color_eyre::eyre;

    use super::*;

    pub fn all<'src>()
    -> impl Parser<'src, &'src str, Vec<(Rotation, Distance)>, extra::Err<Rich<'src, char>>> {
        let rotation = just('L')
            .to(Rotation::Left)
            .or(just('R').to(Rotation::Right));
        let distance = text::int(10).map(|s: &str| s.parse().unwrap());
        let line = rotation.then(distance).then_ignore(just('\n'));
        line.repeated().collect()
    }

    pub fn parse(input: &str) -> eyre::Result<Vec<(Rotation, Distance)>> {
        all()
            .parse(input)
            .into_result()
            .map_err(|errs| eyre!("parsing error {:?}", errs))
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE)?;
    assert_eq!(input.len(), 10);
    Ok(())
}
