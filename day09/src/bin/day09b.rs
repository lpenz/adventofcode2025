// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use day09::*;

struct Polygon {
    pub verticals: Vec<(Pos, Pos)>,
    pub horizontals: Vec<(Pos, Pos)>,
}

impl Polygon {
    pub fn new(points: &[Pos]) -> Polygon {
        // get all xs:
        let mut xs = points.iter().map(|p| p.x).collect::<Vec<_>>();
        xs.sort();
        xs.dedup();
        // for all xs, build the verticals
        let verticals = xs
            .iter()
            .flat_map(|x| {
                let ps = points
                    .iter()
                    .filter(|p| p.x == *x)
                    .cloned()
                    .collect::<Vec<_>>();
                ps.iter()
                    .zip(ps.iter().skip(1))
                    .map(|(a, b)| if a.y < b.y { (*a, *b) } else { (*b, *a) })
                    .collect::<Vec<(Pos, Pos)>>()
            })
            .collect();
        // get all ys:
        let mut ys = points.iter().map(|p| p.y).collect::<Vec<_>>();
        ys.sort();
        ys.dedup();
        // for all ys, build the horizontals
        let horizontals = ys
            .iter()
            .flat_map(|y| {
                let ps = points
                    .iter()
                    .filter(|p| p.y == *y)
                    .cloned()
                    .collect::<Vec<_>>();
                ps.iter()
                    .zip(ps.iter().skip(1))
                    .map(|(a, b)| if a.x < b.x { (*a, *b) } else { (*b, *a) })
                    .collect::<Vec<(Pos, Pos)>>()
            })
            .collect();
        Polygon {
            verticals,
            horizontals,
        }
    }

    pub fn inside(&self, a: &Pos, b: &Pos) -> bool {
        let (xmin, xmax) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
        let (ymin, ymax) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
        let ymin_crosses_vertical = self
            .verticals
            .iter()
            .any(|(t, b)| xmin < t.x && t.x < xmax && t.y <= ymin && ymin <= b.y);
        if ymin_crosses_vertical {
            return false;
        }
        let ymax_crosses_vertical = self
            .verticals
            .iter()
            .any(|(t, b)| xmin < t.x && t.x < xmax && t.y <= ymax && ymax <= b.y);
        if ymax_crosses_vertical {
            return false;
        }
        let xmin_crosses_horizontal = self
            .horizontals
            .iter()
            .any(|(l, r)| ymin < l.y && l.y < ymax && l.x <= xmin && xmin <= r.x);
        if xmin_crosses_horizontal {
            return false;
        }
        let xmax_crosses_horizontal = self
            .horizontals
            .iter()
            .any(|(l, r)| ymin < l.y && l.y < ymax && l.x <= xmax && xmax <= r.x);
        if xmax_crosses_horizontal {
            return false;
        }
        true
    }
}

fn process(input: &str) -> Result<i64> {
    let input = parser::parse(input)?;
    let poly = Polygon::new(&input);
    let mut candidates = input
        .iter()
        .enumerate()
        .flat_map(|(i, a)| input[i + 1..].iter().map(|b| (area(a, b), *a, *b)))
        .collect::<Vec<(i64, Pos, Pos)>>();
    candidates.sort_by(|a, b| b.cmp(a));
    Ok(candidates
        .iter()
        .filter(|(_, a, b)| poly.inside(a, b))
        .max()
        .unwrap()
        .0)
}

#[test]
fn test() -> Result<()> {
    assert_eq!(process(EXAMPLE)?, 24);
    Ok(())
}

fn main() -> Result<()> {
    do_main(process)
}
