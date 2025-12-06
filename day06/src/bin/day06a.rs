// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day06::*;

pub mod parser {
    use aoc::parser::*;

    fn num(input: &str) -> PResult<&str, i64> {
        let (input, _) = many0_spaces.parse(input)?;
        let (input, num) =
            nom::error::context("cannot parse num", map_res(character::i64, i64::try_from))
                .parse(input)?;
        let (input, _) = many0_spaces.parse(input)?;
        Ok((input, num))
    }

    fn numline(input: &str) -> PResult<&str, Vec<i64>> {
        let (input, num) = context("numline err", multi::many1(num)).parse(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, num))
    }

    fn op(input: &str) -> PResult<&str, char> {
        let (input, _) = many0_spaces.parse(input)?;
        let (input, op) =
            nom::error::context("cannot parse num", character::one_of("+*")).parse(input)?;
        let (input, _) = many0_spaces.parse(input)?;
        Ok((input, op))
    }

    fn opline(input: &str) -> PResult<&str, Vec<char>> {
        let (input, op) = context("operation err", multi::many1(op)).parse(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, op))
    }

    fn all(input: &str) -> PResult<&str, (Vec<char>, Vec<Vec<i64>>)> {
        let (input, nums) = multi::many1(numline).parse(input)?;
        let (input, ops) = opline.parse(input)?;
        Ok((input, (ops, nums)))
    }

    pub fn parse(input: &str) -> Result<(Vec<char>, Vec<Vec<i64>>)> {
        aoc::parse_with!(all, input)
    }

    #[test]
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(input.0.len(), 4);
        Ok(())
    }
}

fn process(input: &str) -> Result<i64> {
    let (ops, nums) = parser::parse(input)?;
    Ok(ops
        .into_iter()
        .enumerate()
        .map(|(col, op)| match op {
            '+' => nums.iter().map(|line| line[col]).sum::<i64>(),
            '*' => nums.iter().map(|line| line[col]).product(),
            _ => panic!("unknown operation {}", op),
        })
        .sum())
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 4277556);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
