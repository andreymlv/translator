use std::cmp;

use colored::*;

use crate::text::SourceText;

use super::Diagnostic;

const PREFIX_LENGHT: usize = 80;

pub struct DiagnosticsPrinter<'a> {
    text: &'a SourceText,
    diagnostics: &'a [Diagnostic],
}

impl<'a> DiagnosticsPrinter<'a> {
    pub fn new(text: &'a SourceText, diagnostics: &'a [Diagnostic]) -> Self {
        Self { text, diagnostics }
    }

    pub fn stringify_diagnostic(&self, diagnostic: &'a Diagnostic) -> String {
        let line_index = self.text.line_index(diagnostic.span.start);
        let line = self.text.get_line(line_index);
        let line_start = self.text.line_start(line_index);

        let column = diagnostic.span.start - line_start;
        let (prefix, span, suffix) = Self::get_text_spans(column, diagnostic, line);

        let indent = cmp::min(PREFIX_LENGHT, column);
        let (arrow_pointers, arrow_line) = Self::format_arrow(diagnostic, indent);
        let error_message = format!("{:indent$}+-- {}", "", diagnostic.message, indent = indent);
        format!(
            "{}{}{}\n{}\n{}\n{}",
            prefix,
            span.red(),
            suffix,
            arrow_pointers,
            arrow_line,
            error_message
        )
    }

    pub fn print(&self) {
        for diagnostic in self.diagnostics {
            println!("{}", self.stringify_diagnostic(diagnostic))
        }
    }

    fn get_text_spans(
        column: usize,
        diagnostic: &'a Diagnostic,
        line: &'a str,
    ) -> (&'a str, &'a str, &'a str) {
        let prefix_start = cmp::max(0, column as isize - PREFIX_LENGHT as isize);
        let prefix_end = column;
        let suffix_start = cmp::min(column + diagnostic.span.len(), line.len());
        let suffix_end = cmp::min(suffix_start + PREFIX_LENGHT, line.len());

        let prefix = &line[prefix_start as usize..prefix_end];
        let span = &line[prefix_end..suffix_start];
        let suffix = &line[suffix_start..suffix_end];
        (prefix, span, suffix)
    }

    fn format_arrow(diagnostic: &Diagnostic, indent: usize) -> (String, String) {
        let arrow_pointers = format!(
            "{:indent$}{}",
            "",
            std::iter::repeat('^')
                .take(diagnostic.span.len())
                .collect::<String>(),
            indent = indent
        );
        let arrow_line = format!("{:indent$}|", "", indent = indent);
        "".to_string();
        (arrow_pointers, arrow_line)
    }
}
