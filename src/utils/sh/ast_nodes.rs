type Script<'a> = Vec<CompleteCommand<'a>>;

#[derive(Debug, PartialEq)]
pub struct CompleteCommand<'a> {
    expression: Expression<'a>,
    subshell: bool
}

#[derive(Debug, PartialEq)]
pub struct Expression<'a> {
    seq: Vec<LogicalSeqElem<'a>>,
    term: PipeLine<'a>
}

#[derive(Debug, PartialEq)]
pub struct LogicalSeqElem<'a> {
    op: LogicalOp,
    pipeline: PipeLine<'a>
}

#[derive(Debug, PartialEq)]
pub struct PipeLine<'a> {
    bang: bool,
    pipesequence: Vec<SimpleCommand<'a>>
}

#[derive(Debug, PartialEq)]
pub struct AssignmentWord<'a> {
    name: &'a [u8],
    value: &'a [u8]
}

#[derive(Debug, PartialEq)]
pub struct SimpleCommand<'a> {
    assignment_words: Vec<AssignmentWord<'a>>,
    command_name: Option<Word<'a>>,
    args: Vec<Word<'a>>
}

#[derive(Debug, PartialEq)]
pub struct Word<'a> {
    text: &'a [u8]
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