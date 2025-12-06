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
    use aoc::parser::*;

    use super::*;

    fn num(input: &str) -> PResult<&str, Num> {
        nom::error::context("cannot parse Num", map_res(character::i64, i64::try_from)).parse(input)
    }

    fn rangeline(input: &str) -> PResult<&str, Range> {
        let (input, n1) = context("num err", num).parse(input)?;
        let (input, _) = tag("-").parse(input)?;
        let (input, n2) = context("num err", num).parse(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (n1, n2)))
    }

    fn numline(input: &str) -> PResult<&str, Num> {
        let (input, num) = context("num err", num).parse(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, num))
    }

    fn all(input: &str) -> PResult<&str, (Vec<Range>, Vec<Num>)> {
        let (input, ranges) = multi::many1(rangeline).parse(input)?;
        let (input, _) = character::newline(input)?;
        let (input, nums) = multi::many1(numline).parse(input)?;
        Ok((input, (ranges, nums)))
    }

    pub fn parse(input: &str) -> Result<(Vec<Range>, Vec<Num>)> {
        aoc::parse_with!(all, input)
    }

    #[test]
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(input.0.len(), 4);
        assert_eq!(input.1.len(), 6);
        Ok(())
    }
}
