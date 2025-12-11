// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day11::*;

use std::collections::HashMap;

type Graph = HashMap<Node, Vec<Node>>;

fn dfs(graph: &Graph, node: &Node, paths: &mut usize) {
    if node.0 == "out" {
        *paths += 1;
        return;
    }
    let Some(neighbors) = graph.get(node) else {
        return;
    };
    for neigh in neighbors {
        dfs(graph, neigh, paths);
    }
}

fn process(input: &str) -> Result<usize> {
    let input = parser::parse(input)?;
    let graph = input.into_iter().collect::<HashMap<Node, Vec<Node>>>();
    let mut paths = 0;
    dfs(&graph, &Node("you".parse().unwrap()), &mut paths);
    Ok(paths)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE1)?, 5);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
