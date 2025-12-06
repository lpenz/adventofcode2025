// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day06::*;

pub mod parser {
    use aoc::parser_nom::*;

    fn flip(input: &str) -> String {
        let v0 = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        let mut s = String::default();
        for x in 0..v0[0].len() {
            for line in &v0 {
                s.push(line[v0[0].len() - 1 - x]);
            }
            s.push('\n');
        }
        s
    }

    #[test]
    fn test_flip() {
        assert_eq!(
            flip(day06::EXAMPLE),
            "  4 
431 
623+
    
175 
581 
 32*
    
8   
248 
369+
    
356 
24  
1  *
"
        );
    }

    fn num(input: &str) -> PResult<&str, i64> {
        let (input, _) = many0_spaces.parse(input)?;
        let (input, num) = map_res(character::i64, i64::try_from).parse(input)?;
        Ok((input, num))
    }

    fn op(input: &str) -> PResult<&str, char> {
        let (input, _) = many0_spaces.parse(input)?;
        let (input, op) = multi::many0(character::one_of("+*")).parse(input)?;
        Ok((
            input,
            match op[..] {
                [op] => op,
                [] => ' ',
                _ => panic!("error parsing op"),
            },
        ))
    }

    fn numline(input: &str) -> PResult<&str, (char, i64)> {
        let (input, num) = num.parse(input)?;
        let (input, op) = op.parse(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (op, num)))
    }

    fn emptyline(input: &str) -> PResult<&str, (char, i64)> {
        let (input, _) = many0_spaces.parse(input)?;
        let (input, _) = character::newline(input)?;
        Ok((input, (' ', 0)))
    }

    fn line(input: &str) -> PResult<&str, (char, i64)> {
        branch::alt((numline, emptyline)).parse(input)
    }

    pub fn parse(input: &str) -> Result<Vec<(char, i64)>> {
        let flipped = flip(input);
        aoc::parse_with!(multi::many1(line), flipped)
    }
}

fn process(input: &str) -> Result<i64> {
    let input = parser::parse(input)?;
    let mut result = 0;
    let mut nums = Vec::<i64>::default();
    for (op, num) in input {
        if num > 0 {
            nums.push(num);
        }
        if op == '+' {
            result += nums.drain(0..).sum::<i64>();
        } else if op == '*' {
            result += nums.drain(0..).product::<i64>();
        }
    }
    Ok(result)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 3263827);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
