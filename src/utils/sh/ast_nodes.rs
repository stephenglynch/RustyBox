use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::unix::prelude::OsStrExt;
use std::os::unix::prelude::OsStringExt;
use std::error::Error;
use std::process::Command;
use std::process::Stdio;
use std::collections::HashMap;
use core::slice::Iter;
use nom::multi::many0;
use super::expansions::expandable;
use log::*;


#[derive(Debug, PartialEq)]
pub struct VarValue {
    pub value: Vec<u8>,
    pub export: bool
}

impl VarValue {
    pub fn new(value: Vec<u8>, export: bool) -> Self {
        VarValue {value: value, export: export}
    }
    
    pub fn new_no_export(value: Vec<u8>) -> Self {
        Self::new(value, false)
    }

    pub fn new_export(value: Vec<u8>) -> Self {
        Self::new(value, true)
    }
}

#[derive(Debug, PartialEq)]
pub struct ExecEnv {
    pub env: HashMap<OsString, VarValue>
}

pub type Script<'a> = Vec<CompleteCommand<'a>>;

#[derive(Debug, PartialEq)]
pub struct CompleteCommand<'a> {
    pub expression: Expression<'a>,
    pub subshell: bool
}

impl<'a> CompleteCommand<'a> {
    pub fn execute(&self, ev: &mut ExecEnv) -> Result<i32, Box<dyn Error>> {
        self.expression.execute(ev)
    }   
}

#[derive(Debug, PartialEq)]
pub struct Expression<'a> {
    pub seq: Vec<LogicalSeqElem<'a>>,
    pub term: PipeLine<'a>
}

impl<'a> Expression<'a> {
    fn execute(&self, ev: &mut ExecEnv) -> Result<i32, Box<dyn Error>> {
        let mut ret = self.term.execute_pipeline(ev)?;
        for l in self.seq.iter() {
            let op_res = match l.op {
                LogicalOp::And => ret == 0,
                LogicalOp::Or => ret != 0
            };
            
            if op_res {
                ret = l.pipeline.execute_pipeline(ev)?;
            } else {
                break;
            }
        }

        Ok(ret)
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
    fn execute_pipeline(&self, ev: &mut ExecEnv) -> Result<i32, Box<dyn Error>> {
        let mut children = vec![];
        let mut commands: Vec<_> = self.pipesequence
            .iter()
            .map(|cmd| cmd.setup_command(ev))
            .collect();

        commands.reverse();

        let mut cmd_left = commands.pop().expect("Failed to pop command");
        for mut cmd_right in commands {            
            match (&mut cmd_left, &mut cmd_right) {
                (Some(lcmd), Some(rcmd)) => {
                    lcmd.stdout(Stdio::piped());
                    let mut child = lcmd.spawn()?;
                    rcmd.stdin(child.stdout.take().expect("Failed to create pipe"));
                    children.push(Some(child));
                },
                (Some(lcmd), None) => {
                    lcmd.stdout(Stdio::null());
                    let child = lcmd.spawn()?;
                    children.push(Some(child));
                },
                (None, Some(_)) => (),
                (None, None) => ()
            };

            cmd_left = cmd_right;
        }

        if let Some(mut final_command) = cmd_left {
            let child = final_command.spawn()?;
            children.push(Some(child));
        }
        

        let mut final_exit_val = 0;
        for child_opt in children {
            final_exit_val = match child_opt {
                Some(mut child) => child.wait()?.code().unwrap(),
                None => 0
            };
        }   

        Ok(final_exit_val)
    }
}

pub type AssignmentWords = Vec<(OsString, OsString)>;

#[derive(Debug, PartialEq)]
pub struct SimpleCommand<'a> {
    pub assignment_words: AssignmentWords,
    pub words: Vec<Word<'a>>
}

impl<'a> SimpleCommand<'a> {
    fn command_name<'b>(&self, fields: &'b Vec<Vec<u8>>) -> Option<&'b Vec<u8>> {
        if fields.len() > 0 {
            Some(&fields[0])
        } else {
            None
        }
    }

    fn args<'b>(&self, fields: &'b Vec<Vec<u8>>) -> Iter<'b, Vec<u8>> {
        fields[1..].iter()
    }

    fn save_variables(&self, ev: &mut ExecEnv) {
        for (name, val) in self.assignment_words.iter() {
            ev.env.insert(name.clone(), VarValue { 
                value: val.clone().into_vec(), 
                export: false
            });
        }
    }

    fn fields(&self, ev: &mut ExecEnv) -> Vec<Vec<u8>> {
        // Expand each expandable token e.g. variables, text, commands
        info!("{:?}", self.words);
        let mut combined = vec![];
        for w in self.words.iter() {
            info!("w = {:?}", w);
            let (_, exps) = many0(expandable)(w.text).unwrap();
            info!("exps = {:?}", exps);
            let init = b"".to_vec();
            let mut expanded = exps.into_iter().fold(init, |mut acc, ex| {
                let mut expanded = ex.expand(ev);
                acc.append(&mut expanded);
                acc
            });
            combined.append(&mut expanded);
        }

        info!("words = {:?}", self.words);
        info!("combined = {:?}", combined);

        combined.split(|&b| b == b' ').map(|f| f.to_vec()).collect()
    }
 
    fn setup_command(&self, ev: &mut ExecEnv) -> Option<Command> {
        let fields = self.fields(ev);

        info!("{:?}", fields);

        let command_name = match self.command_name(&fields) {
            Some(command_name) => OsString::from_vec(command_name.clone()),
            None => {
                self.save_variables(ev);
                return None
            }
        };

        // Convert Vec<u8> into Iter of OsStr
        let osargs = self.args(&fields).map(|arg| {
            OsStr::from_bytes(&arg)
        });

        let mut cmd: Command = Command::new(&command_name);
        cmd.args(osargs);

        // Pass environment if variables marked for export
        for (name, val) in &ev.env {
            if val.export{
                cmd.env(name, OsStr::from_bytes(&val.value));
            }
        }

        // Pass assignment words
        for (name, val) in self.assignment_words.iter() {
            cmd.env(name, val);
        }

        Some(cmd)
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

#[allow(dead_code)]
pub type IoNumber = u32;

#[derive(Debug, PartialEq)]
pub enum SeperatorOp {
    Async,
    Seq
}