// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::collections::HashMap;
use std::str::FromStr;

use cached::SizedCache;
use cached::proc_macro::cached;

use day11::*;

type Graph = HashMap<Node, Vec<Node>>;

#[cached(
    ty = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(10000) }",
    convert = r#"{ format!("{}~{}", node.0, dst.0) }"#
)]
fn dfs(graph: &Graph, node: &Node, dst: &Node) -> usize {
    if node == dst {
        return 1;
    }
    let Some(neighbors) = graph.get(node) else {
        return 0;
    };
    neighbors.iter().map(|neigh| dfs(graph, neigh, dst)).sum()
}

fn process(input: &str) -> Result<usize> {
    let input = parser::parse(input)?;
    let graph = input.into_iter().collect::<HashMap<Node, Vec<Node>>>();
    let fft_paths = dfs(&graph, &Node::from_str("svr")?, &Node::from_str("fft")?);
    let dac_paths = dfs(&graph, &Node::from_str("fft")?, &Node::from_str("dac")?);
    let out_paths = dfs(&graph, &Node::from_str("dac")?, &Node::from_str("out")?);
    Ok(dac_paths * fft_paths * out_paths)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE2)?, 2);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
