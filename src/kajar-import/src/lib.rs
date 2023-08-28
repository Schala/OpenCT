use nom::{
    character::complete::multispace0, combinator::value, error::ParseError, sequence::delimited,
    IResult,
};

use std::io::{self, Read};

//pub(crate) mod markup;
//pub(crate) mod resbin;
//mod blowfish;
mod tim;

/// Converts a 4-byte string into a 32-bit big endian integer.
/// Byte strings longer than 4 bytes are truncated.
#[macro_export]
macro_rules! tag {
    ($b4: literal) => {
        u32::from_be_bytes([$b4[3], $b4[2], $b4[1], $b4[0]])
    };
}

/// Reads a null-terminated string from a buffer
pub(crate) fn read_cstr(mut buf: impl Read) -> io::Result<String> {
    let mut s = String::new();
    let mut b = [0; 1];

    loop {
        buf.read_exact(&mut b)?;
        if b[0] != 0 {
            s.push(b[0] as char);
        } else {
            break;
        }
    }

    Ok(s)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub(crate) fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
