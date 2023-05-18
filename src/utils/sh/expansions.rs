use std::ffi::OsString; 
use std::os::unix::prelude::OsStringExt;
use nom::{
    bytes::complete::{tag, take_until, is_a},
    branch::alt,
    IResult
};
use crate::utils::sh::ast_nodes::ExecEnv;

pub enum Expandable {
    Text(Vec<u8>),
    VariableSub(Vec<u8>),
    CommandSub(Vec<Expandable>),
    Arithmetic(Vec<Expandable>)
}

impl Expandable {
    fn expand(self, ev: ExecEnv) -> Option<Vec<u8>> {
        match self {
            Self::Text(s) => Some(s),
            Self::VariableSub(s) => {
                let k = OsString::from_vec(s);
                Some(ev.env.get(&k)?.value.clone())
            },
            _ => unimplemented!()
        }
    }
}

fn expandable<'a>(input: &'a [u8]) -> IResult<&'a [u8], Expandable> {
    alt((
        text,
        variable
    ))(input)
}

fn text<'a>(input: &'a [u8]) -> IResult<&'a [u8], Expandable> {
    let (input, t) = take_until(b"$".as_ref())(input)?;
    Ok((input, Expandable::Text(t.to_vec())))
}

fn variable<'a>(input: &'a [u8]) -> IResult<&'a [u8], Expandable> {
    let name_char_set = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
    let (input, _) = tag(b"$".as_ref())(input)?;
    let (input, name) = is_a(name_char_set.as_ref())(input)?;
    Ok((input, Expandable::Text(name.to_vec())))
}
