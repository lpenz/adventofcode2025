// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::str::FromStr;

use tinystr::TinyAsciiStr;

pub use aoc::*;

pub const EXAMPLE1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

pub const EXAMPLE2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node(pub TinyAsciiStr<3>);

impl FromStr for Node {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.parse().wrap_err("could not convert to TinyAsciiStr")?,
        ))
    }
}

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    pub fn parse(input: &str) -> Result<Vec<(Node, Vec<Node>)>> {
        let node = none_of(" ")
            .repeated()
            .exactly(3)
            .collect::<String>()
            .try_map(do_parse);
        chumsky_parse(
            input,
            node.then_ignore(just(": "))
                .then(node.separated_by(just(" ")).collect::<Vec<Node>>())
                .then_ignore(just("\n"))
                .repeated()
                .collect(),
        )
    }

    #[test]
    fn test1() -> Result<()> {
        let input = parse(crate::EXAMPLE1)?;
        assert_eq!(input.len(), 10);
        Ok(())
    }

    #[test]
    fn test2() -> Result<()> {
        let input = parse(crate::EXAMPLE2)?;
        assert_eq!(input.len(), 13);
        Ok(())
    }
}
