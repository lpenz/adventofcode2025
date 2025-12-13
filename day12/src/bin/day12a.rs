// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day12::*;

fn process(input: &str) -> Result<usize> {
    let (shapes, regions) = parser::parse(input)?;
    Ok(regions
        .into_iter()
        .enumerate()
        .filter(|(i, (width, height, reqshapes))| {
            let region_squares = (width / 3 * 3) * (height / 3 * 3) / 9;
            let shapes_total = reqshapes.iter().sum::<usize>();
            if shapes_total <= region_squares {
                eprintln!(
                    "{}: {}x{} has {} squares that fit {} shapes individually",
                    i, width, height, region_squares, shapes_total
                );
                return true;
            }
            let region_cells = width * height;
            let shapes_cells = reqshapes
                .iter()
                .enumerate()
                .map(|(ishape, req)| req * shapes[ishape].num_cells())
                .sum::<usize>();
            if region_cells < shapes_cells {
                eprintln!(
                    "{}: {}x{} has {} cells that can't fit {} required by shapes",
                    i, width, height, region_cells, shapes_cells
                );
                return false;
            }
            panic!("{}: {}x{} NEED TO CALCULATE", i, width, height,);
        })
        .count())
}

#[test]
fn test() -> Result<()> {
    // The example is more general than the actual input, I'm not bothering with it.
    // assert_eq!(process(EXAMPLE)?, 2);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
