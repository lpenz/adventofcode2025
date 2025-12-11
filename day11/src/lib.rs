// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use tinystr::TinyAsciiStr;

pub use aoc::*;

pub const EXAMPLE: &str = "aaa: you hhh
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node(pub TinyAsciiStr<3>);

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    pub fn parse(input: &str) -> Result<Vec<(Node, Vec<Node>)>> {
        let node = none_of(" ")
            .repeated()
            .exactly(3)
            .collect::<String>()
            .map(|s| Node(s.parse::<TinyAsciiStr<3>>().unwrap()));
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
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(input.len(), 10);
        Ok(())
    }
}
