// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day11::*;

fn process(input: &str) -> Result<u8> {
    let input = parser::parse(input)?;
    println!("digraph {{");
    println!("  svr [ style=filled fillcolor=red ]");
    println!("  dac [ style=filled fillcolor=yellow ]");
    println!("  fft [ style=filled fillcolor=yellow ]");
    println!("  out [ style=filled fillcolor=blue ]");
    for (n1, nodes) in input.into_iter() {
        for n2 in nodes.into_iter() {
            println!("  \"{}\" -> \"{}\"", n1.0, n2.0);
        }
    }
    println!("}}");
    Ok(0)
}

fn main() -> Result<()> {
    do_main(process)
}
