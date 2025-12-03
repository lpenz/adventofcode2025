// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

pub type Jolt = u64;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn line(input: &str) -> PResult<&str, Vec<Jolt>> {
        let (input, digits) = multi::many0(map_res(digit1, Jolt::try_from)).parse(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, digits))
    }

    pub fn parse(input: &str) -> Result<Vec<Vec<Jolt>>> {
        aoc::parse_with!(multi::many1(line), input)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE)?;
    assert_eq!(input.len(), 4);
    Ok(())
}
