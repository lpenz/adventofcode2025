// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use aoc::*;

pub const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

#[derive(enumchar::EnumChar, Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
pub enum Light {
    #[char('#')]
    On,
    #[default]
    #[char('.')]
    Off,
}

impl std::ops::Neg for Light {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Light::On => Light::Off,
            Light::Off => Light::On,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Lights(pub Vec<Light>);

impl Lights {
    pub fn new(size: usize) -> Lights {
        Lights(vec![Light::default(); size])
    }

    pub fn toggle(&mut self, button: &Button) {
        for &i in &button.0 {
            self.0[i] = -self.0[i];
        }
    }

    pub fn new_pressed(&self, button: &Button) -> Lights {
        let mut lights = self.clone();
        lights.toggle(button);
        lights
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Button(pub Vec<usize>);

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Jolts(pub Vec<usize>);

impl Jolts {
    pub fn new(size: usize) -> Jolts {
        Jolts(vec![0; size])
    }

    pub fn press(&mut self, button: &Button) {
        for &i in &button.0 {
            self.0[i] += 1;
        }
    }

    pub fn new_pressed(&self, button: &Button) -> Jolts {
        let mut jolts = self.clone();
        jolts.press(button);
        jolts
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub mod parser {
    use aoc::parser_chumsky::*;
    use chumsky::prelude::*;

    use super::*;

    type Line = (Lights, Vec<Button>, Jolts);

    pub fn parse(input: &str) -> Result<Vec<Line>> {
        let lights = just('[')
            .ignore_then(enum_char(".#").repeated().collect().map(Lights))
            .then_ignore(just(']'))
            .then_ignore(just(' '));
        let wiring = just('(')
            .ignore_then(number().separated_by(just(',')).collect().map(Button))
            .then_ignore(just(')'))
            .then_ignore(just(' '));
        let joltage = just('{')
            .ignore_then(number().separated_by(just(',')).collect().map(Jolts))
            .then_ignore(just('}'))
            .then_ignore(just('\n'));
        let line = lights
            .then(wiring.repeated().collect())
            .then(joltage)
            .map(|((a, b), c)| (a, b, c));
        chumsky_parse(input, line.repeated().collect())
    }

    #[test]
    fn test() -> Result<()> {
        let input = parse(crate::EXAMPLE)?;
        assert_eq!(input.len(), 3);
        Ok(())
    }
}
