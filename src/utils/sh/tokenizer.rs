use nom::{error::ErrorKind, IResult};

#[derive(Debug, PartialEq)]
enum TokenType {
    Word,
    Op
}

fn is_op_initial(c: u8) -> bool {
    b"&|;<>".contains(&c)
}

fn is_op_character(c: u8) -> bool {
    b"&|;<>-".contains(&c)
}

fn is_blank(c: u8) -> bool {
    c == b' ' || c == b'\t'
}

fn is_newline(c: u8) -> bool {
    c == b'\n'
}

fn is_comment(c: u8) -> bool {
    c == b'#'
}

fn after_comment(s: &[u8]) -> &[u8] {
    for (i, c) in s.into_iter().enumerate() {
        if is_newline(*c) {
            return &s[(i+1)..];
        }
    }

    return b"";
}

fn raw_token(input: &[u8]) -> IResult<&[u8], (TokenType, &[u8])> {
    // Initialise with maximum size of token
    let mut is_operator = false;
    let mut tok_start = 0;
    let mut tok_len = 0;
    let mut active_tok = false;
    for (i, c) in input.into_iter().enumerate() {

        active_tok = tok_len - tok_start != 0;

        // Tokenizer rule 2
        if is_operator {
            // Tokenizer rule 3
            if is_op_character(*c) {
                tok_len += 1;
                continue;
            } else {
                is_operator = false; // Finished processing operator
                let tok_end = tok_len - tok_start;
                return Ok((&input[i..], (TokenType::Op, &input[tok_start..tok_end])))
            }
        }

        // TODO: Tokenizer rule 4
        // TODO: Tokenizer rule 5

        // Tokenizer rule 6
        if is_op_initial(*c) {
            is_operator = true;
            if active_tok {
                let tok_end = tok_len - tok_start;
                return Ok((&input[i..], (TokenType::Word, &input[tok_start..tok_end])))
            } else {
                tok_len += 1;
                continue;
            }
        }

        // Tokenizer rule 7
        if is_newline(*c) {
            tok_len += 1;
            let tok_end = tok_len - tok_start;
            return Ok((&input[(i+1)..], (TokenType::Word, &input[tok_start..tok_end])))
        }

        // Tokenizer rule 8
        if is_blank(*c) {
            if tok_len > 0 {
                let tok_end = tok_len - tok_start;
                return Ok((&input[(i+1)..], (TokenType::Word, &input[tok_start..tok_end])))
            } else {
                continue;
            }
        } 

        // Tokenizer rule 9
        if tok_len > 0 {
            tok_len += 1;
            continue;
        }

        // TODO: Tokenizer rule 10
        if is_comment(*c) {
            let rest = after_comment(&input[i..]);
            return Ok((rest, (TokenType::Word, b"\n")))
        }

        // Tokenizer rule 11
        tok_len += 1;
    }

    // Tokenizer rule 1
    let tok_end = tok_len - tok_start;
    if active_tok {
        Ok((b"", (TokenType::Word, &input[tok_start..tok_end])))
    } else {
        Err(nom::Err::Error(
            nom::error::Error::new(b"", ErrorKind::Fail)
        ))
    }
}

pub fn word(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, (token_type, tok)) = raw_token(input)?;
    if token_type == TokenType::Word {
        return Ok((rest, tok))
    } else {
        Err(nom::Err::Error(
            nom::error::Error::new(input, ErrorKind::Fail)
        ))
    }
}

pub fn op(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, (token_type, tok)) = raw_token(input)?;
    if token_type == TokenType::Op {
        return Ok((rest, tok))
    } else {
        Err(nom::Err::Error(
            nom::error::Error::new(input, ErrorKind::Fail)
        ))
    }
}


mod tests {
    use nom::IResult;
    use nom::error;
    use std::str;
    use super::*;

    macro_rules! test_token {
        ( $test_name:ident, $test_string:expr, $expected_op:expr, $expected_tok:expr, $expected_remain:expr ) => {
            #[test]
            fn $test_name() {
                let test_string = $test_string.as_bytes().to_vec();
                let (remaining, (op, tok)) = raw_token(&test_string).unwrap();
                let remaining = str::from_utf8(&remaining).unwrap();
                let tok = str::from_utf8(&tok).unwrap();
                assert_eq!(op, $expected_op);
                assert_eq!(tok, $expected_tok);
                assert_eq!(remaining, $expected_remain);
            }
        }
    }

    test_token!(test_end, "foo", TokenType::Word, "foo", "");
    test_token!(test_space, "foo bar", TokenType::Word, "foo", "bar");
    test_token!(test_op, "foo|bar", TokenType::Word, "foo", "|bar");
    test_token!(test_op2, "|bar", TokenType::Op, "|", "bar");
    test_token!(test_op3, "foo&bar", TokenType::Word, "foo", "&bar");
    test_token!(test_op4, "&bar", TokenType::Op, "&", "bar");
    test_token!(test_op5, "foo;bar", TokenType::Word, "foo", ";bar");
    test_token!(test_op6, ";bar", TokenType::Op, ";", "bar");
    test_token!(test_op7, "foo&&bar", TokenType::Word, "foo", "&&bar");
    test_token!(test_op8, "&&bar", TokenType::Op, "&&", "bar");
    test_token!(test_op9, "foo||bar", TokenType::Word, "foo", "||bar");
    test_token!(test_op10, "||bar", TokenType::Op, "||", "bar");
    test_token!(test_op11, "foo || bar", TokenType::Word, "foo", "|| bar");
    test_token!(test_op12, "|| bar", TokenType::Op, "||", " bar");
    test_token!(test_newline, "foo\nbar", TokenType::Word, "foo\n", "bar");
    test_token!(test_newline2, "\n\nfoo", TokenType::Word, "\n", "\nfoo");
    test_token!(test_comment, "#foo\nbar", TokenType::Word, "\n", "bar");
    test_token!(test_comment2, "foo#bar\n", TokenType::Word, "foo#bar\n", "");

    #[test]
    fn test_empty() {
        let test_string = b"";
        let blah = b"".as_ref();
        let expected: IResult<&[u8], (TokenType, &[u8]), error::Error<&[u8]>> = Err(
            nom::Err::Error(error::Error::new(blah, error::ErrorKind::Fail))
        );
        let actual_result = raw_token(test_string);
        assert_eq!(actual_result, expected);
    }

    #[test]
    fn test_word_tok() {
        let test_string = "foo&&bar";
        let expected_tok = "foo";
        let expected_remain = "&&bar";
        let test_string = test_string.as_bytes().to_vec();
        let (remaining, tok) = word(&test_string).unwrap();
        let remaining = str::from_utf8(&remaining).unwrap();
        let tok = str::from_utf8(&tok).unwrap();
        assert_eq!(tok, expected_tok);
        assert_eq!(remaining, expected_remain);
    }

    #[test]
    fn test_op_tok() {
        let test_string = "&&bar";
        let expected_tok = "&&";
        let expected_remain = "bar";
        let test_string = test_string.as_bytes().to_vec();
        let (remaining, tok) = op(&test_string).unwrap();
        let remaining = str::from_utf8(&remaining).unwrap();
        let tok = str::from_utf8(&tok).unwrap();
        assert_eq!(tok, expected_tok);
        assert_eq!(remaining, expected_remain);
    }
}