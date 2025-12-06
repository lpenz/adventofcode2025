// Copyright (C) 2025 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

pub use color_eyre::Result;

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

#[macro_export]
macro_rules! parse_with_chumsky {
    ($parser:expr, $input:ident) => {{
        match $parser.parse($input).into_result() {
            Ok(v) => Ok(v),
            Err(e) => {
                eprint_rich_error($input, &e)?;
                Err(eyre!("parsing error {:?}", &e))
            }
        }
    }};
}
