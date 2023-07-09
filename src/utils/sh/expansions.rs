use std::ffi::OsString; 
use std::os::unix::prelude::OsStringExt;
use nom::{
    bytes::complete::{tag, take_till, is_a},
    branch::alt
};

use super::{error::*, ast_nodes::VarValue};
use super::ast_nodes::ExecEnv;
use log::*;

#[derive(Debug)]
pub enum Expandable {
    Text(Vec<u8>),
    VariableSub(Vec<u8>),
    CommandSub(Vec<Expandable>),
    Arithmetic(Vec<Expandable>)
}

impl Expandable {
    pub fn expand(self, ev: &ExecEnv) -> Vec<u8> {
        match self {
            Self::Text(s) => s,
            Self::VariableSub(s) => {
                let k = OsString::from_vec(s);
                let empty_var = VarValue::new_no_export(vec![]);
                let v = ev.env.get(&k).unwrap_or(&empty_var);
                v.value.clone()
            },
            _ => unimplemented!()
        }
    }
}

pub fn expandable<'a>(input: &'a [u8]) -> RbResult<&'a [u8], Expandable> {
    info!("input = {:?}", input);
    alt((
        variable,
        text
    ))(input)
}

fn text<'a>(input: &'a [u8]) -> RbResult<&'a [u8], Expandable> {
    info!("text input = {:?}", input);
    let (input, t) = take_till(|c| c == b'$')(input)?;
    info!("text t = {:?}", t);
    Ok((input, Expandable::Text(t.to_vec())))
}

fn variable<'a>(input: &'a [u8]) -> RbResult<&'a [u8], Expandable> {
    let name_char_set = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
    let (input, _) = tag(b"$".as_ref())(input)?;
    let (input, name) = is_a(name_char_set.as_ref())(input)?;
    Ok((input, Expandable::Text(name.to_vec())))
}
