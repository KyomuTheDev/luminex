extern crate enum_index;

use pad::{Alignment, PadStr};
use crate::ast;

pub fn get_error_message(
    error: ast::ParserError,
    line: usize,
    line_pos: usize,
    literal: String,
) -> String {
    let fill = line_pos + line.to_string().len() + 2;

    format!(
        "
------------------------------------------------
{}: {}
{}
{}
------------------------------------------------
",
        line,
        literal,
        "^".pad_to_width_with_alignment(fill, Alignment::Right),
        error.to_string()
    )
}
