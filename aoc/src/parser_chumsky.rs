// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::Result;
pub use color_eyre::eyre::eyre;

use chumsky::prelude::*;

use ariadne::Color;
use ariadne::Label;
use ariadne::Report;
use ariadne::ReportKind;

pub fn eprint_rich_error<'a>(src: &'a str, errs: &'a [Rich<'a, char>]) -> Result<()> {
    let filename = String::new();
    let mut builder = Report::build(ReportKind::Error, (filename.clone(), 0..0))
        .with_message(errs[0].to_string());
    for err in errs {
        let span = err.span(); // Range<usize>
        let spanrange = span.start()..span.end();
        let reason = err.reason(); // RichReason
        builder = builder.with_label(
            Label::new((filename.clone(), spanrange))
                .with_message(reason)
                .with_color(Color::Red),
        );
    }
    builder
        .finish()
        .print((filename, ariadne::Source::from(src)))?;
    Ok(())
}

pub fn chumsky_parse<'src, P, R>(input: &'src str, parser: P) -> Result<R>
where
    P: Parser<'src, &'src str, R, extra::Err<Rich<'src, char>>>,
{
    match parser.parse(input).into_result() {
        Ok(v) => Ok(v),
        Err(e) => {
            eprint_rich_error(input, &e)?;
            Err(eyre!("parsing error {:?}", &e))
        }
    }
}

pub fn digit1<'src>() -> impl Parser<'src, &'src str, u8, extra::Err<Rich<'src, char>>> {
    one_of("0123456789").try_map(|c: char, span| {
        c.to_digit(10)
            .map(|d| d as u8)
            // Impossible error, mostly here as an example:
            .ok_or_else(|| Rich::custom(span, eyre!("invalid digit")))
    })
}

pub fn number<'src, Num>() -> impl Parser<'src, &'src str, Num, extra::Err<Rich<'src, char>>>
where
    Num: std::str::FromStr,
    <Num as std::str::FromStr>::Err: std::fmt::Display,
{
    text::int(10).try_map(|s: &str, span| {
        Num::from_str(s).map_err(|e| Rich::custom(span, eyre!("error parsing number {}: {}", s, e)))
    })
}

pub fn vecvec<'src, Cell>(
    chars: &'src str,
) -> impl Parser<'src, &'src str, Vec<Vec<Cell>>, extra::Err<Rich<'src, char>>>
where
    Cell: TryFrom<char>,
    <Cell as std::convert::TryFrom<char>>::Error: std::fmt::Display,
{
    let cell = one_of(chars).try_map(|c: char, span| {
        Cell::try_from(c).map_err(|e| Rich::custom(span, eyre!("error parsing cell {}: {}", c, e)))
    });
    let line = cell.repeated().collect().then_ignore(just('\n'));
    line.repeated().collect::<Vec<Vec<Cell>>>()
}
