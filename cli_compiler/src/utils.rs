extern crate enum_index;

use enum_index::*;
use pad::{Alignment, PadStr};
use std::fmt::{Debug, Display};

#[derive(EnumIndex, IndexEnum, Debug)]
enum ErrorMessage {
    ExpectedIdentifier,
    InvalidToken,
    UnexpectedToken,
    TestingCoolToken,
}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut index = 0;

        let str = format!("{:?}", self);

        for c in str.chars() {
            if c.is_uppercase() && index > 0 {
                write!(f, "{}", " ")?;
            }
            write!(f, "{}", c)?;

            index += 1;
        }

        Ok(())
    }
}

pub fn get_error_message(error: usize, line: usize, line_pos: usize, literal: String) -> String {
    let err = ErrorMessage::index_enum(error).unwrap();
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
        err.to_string()
    )
}
