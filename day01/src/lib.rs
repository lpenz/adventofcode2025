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

#[derive(EnumChar, Debug, PartialEq, Eq)]
pub enum Rotation {
    #[char('L')]
    Left,
    #[char('R')]
    Right,
}

pub type Distance = i32;
pub type Position = i32;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn rotation(input: &str) -> PResult<&str, Rotation> {
        nom::error::context(
            "cannot parse rotation",
            map_res(character::one_of("LR"), Rotation::try_from),
        )
        .parse(input)
    }

    fn distance(input: &str) -> PResult<&str, Distance> {
        nom::error::context(
            "cannot parse distance",
            map_res(character::i32, i32::try_from),
        )
        .parse(input)
    }

    fn line(input: &str) -> PResult<&str, (Rotation, Distance)> {
        let (input, rot) = rotation(input)?;
        let (input, dist) = distance(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (rot, dist)))
    }

    pub fn parse(input: &str) -> Result<Vec<(Rotation, Distance)>> {
        aoc::parse_with!(multi::many1(line), input)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE)?;
    assert_eq!(input.len(), 10);
    Ok(())
}
