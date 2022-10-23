use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::unix::prelude::OsStrExt;
use std::os::unix::prelude::OsStringExt;
use std::process::Command;
use std::collections::HashMap;
use core::slice::Iter;

#[derive(Debug, PartialEq)]
pub struct ExecEnv {
    pub argv: Vec<u8>,
    pub env: HashMap<u8, u8>
}

pub type Script<'a> = Vec<CompleteCommand<'a>>;

#[derive(Debug, PartialEq)]
pub struct CompleteCommand<'a> {
    pub expression: Expression<'a>,
    pub subshell: bool
}

impl<'a> CompleteCommand<'a> {
    pub fn execute(&self, ev: &ExecEnv) -> i32 {
        self.expression.execute(&ev)
    }   
}

#[derive(Debug, PartialEq)]
pub struct Expression<'a> {
    pub seq: Vec<LogicalSeqElem<'a>>,
    pub term: PipeLine<'a>
}

impl<'a> Expression<'a> {
    fn execute(&self, ev: &ExecEnv) -> i32 {
        self.term.execute(&ev)
    }
}

#[derive(Debug, PartialEq)]
pub struct LogicalSeqElem<'a> {
    pub op: LogicalOp,
    pub pipeline: PipeLine<'a>
}

#[derive(Debug, PartialEq)]
pub struct PipeLine<'a> {
    pub bang: bool,
    pub pipesequence: Vec<SimpleCommand<'a>>
}

impl<'a> PipeLine<'a> {
    fn execute(&self, ev: &ExecEnv) -> i32 {
        let mut exit_val = 0;
        for cmd in self.pipesequence.iter() {
            exit_val = cmd.execute(&ev);
        }

        if self.bang {
            !exit_val
        } else {
            exit_val
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AssignmentWord {
    pub name: Vec<u8>,
    pub value: Vec<u8>
}

#[derive(Debug, PartialEq)]
pub struct SimpleCommand<'a> {
    pub assignment_words: Vec<AssignmentWord>,
    pub words: Vec<Word<'a>>
}

impl<'a> SimpleCommand<'a> {
    fn command_name(&self) -> Option<&Word> {
        if self.words.len() > 0 {
            return Some(&self.words[0])
        } else {
            return None
        }
    }

    fn args(&self) -> Iter<'a, Word> {
        return self.words[1..].iter()
    }
 
    fn execute(&self, _ev: &ExecEnv) -> i32 {

        let command_name = match self.command_name() {
            Some(command_name) => OsString::from_vec(command_name.eval()),
            None => return 0
        };

        // Convert Vec<u8> into Iter of OsStr
        let args: Vec<Vec<u8>> = self.args().map(|w| {
            w.eval()
        }).collect();
        let osargs = args.iter().map(|arg| {
            OsStr::from_bytes(&arg)
        });

        // Convert env to 
        Command::new(&command_name)
            .args(osargs)
            .status().unwrap()
            .code().unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub struct Word<'a> {
    pub text: &'a [u8]
}

impl<'a> Word<'a> {
    pub fn new(text: &'a [u8]) -> Word<'a> {
        Word { text: text }
    }

    pub fn eval(&self) -> Vec<u8> {
        self.text.to_vec()
    }
}

#[derive(Debug, PartialEq)]
pub enum LogicalOp {
    Or,
    And
}

#[derive(Debug, PartialEq)]
pub enum RedirectionOp {
    Less,
    LessAnd,
    Great,
    GreatAnd,
    DGreat,
    LessGreat,
    Clobber
}

#[derive(Debug, PartialEq)]
pub enum IoHereOp {
    DLess,
    DLessDash
}

pub type IoNumber = u32;

#[derive(Debug, PartialEq)]
pub enum SeperatorOp {
    Async,
    Seq
}