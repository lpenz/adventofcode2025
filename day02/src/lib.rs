// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n";

pub type Pid = u64;

pub mod parser {
    use aoc::parser::*;

    use super::*;

    fn pid(input: &str) -> PResult<&str, Pid> {
        nom::error::context(
            "cannot parse product ID",
            map_res(character::u64, Pid::try_from),
        )
        .parse(input)
    }

    fn range(input: &str) -> PResult<&str, (Pid, Pid)> {
        let (input, start) = context("err parsing start", pid).parse(input)?;
        let (input, _) = character::char('-')(input)?;
        let (input, end) = context("err parsing end", pid).parse(input)?;
        Ok((input, (start, end)))
    }

    fn line(input: &str) -> PResult<&str, Vec<(Pid, Pid)>> {
        let (input, ranges) = multi::separated_list1(tag(","), range).parse(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, ranges))
    }

    pub fn parse(input: &str) -> Result<Vec<(Pid, Pid)>> {
        aoc::parse_with!(line, input)
    }
}

#[test]
fn test() -> Result<()> {
    let input = parser::parse(EXAMPLE)?;
    assert_eq!(input.len(), 11);
    Ok(())
}
