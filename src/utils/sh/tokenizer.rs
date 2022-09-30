

#[derive(Debug, PartialEq)]
enum Token {
    Word(Vec<u8>),
    Op(Vec<u8>)
}

fn raw_token(input: &[u8]) -> IResult<&[u8], Token> {
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
                return Ok((&input[i..], Token::Word(input[tok_start..tok_end].to_vec())))
            }
        }

        // TODO: Tokenizer rule 4
        // TODO: Tokenizer rule 5

        // Tokenizer rule 6
        if is_op_initial(*c) {
            is_operator = true;
            if active_tok {
                let tok_end = tok_len - tok_start;
                return Ok((&input[i..], Token::Word(input[tok_start..tok_end].to_vec())))
            } else {
                tok_len += 1;
                continue;
            }
        }

        // Tokenizer rule 7
        if is_newline(*c) {
            tok_len += 1;
            let tok_end = tok_len - tok_start;
            return Ok((&input[(i+1)..], Token::Word(input[tok_start..tok_end].to_vec())))
        }

        // Tokenizer rule 8
        if is_blank(*c) {
            if tok_len > 0 {
                let tok_end = tok_len - tok_start;
                return Ok((&input[(i+1)..], Token::Word(input[tok_start..tok_end].to_vec())))
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
            return Ok((rest, Token::Word(b"\n".to_vec())))
        }

        // Tokenizer rule 11
        tok_len += 1;
    }

    // Tokenizer rule 1
    let tok_end = tok_len - tok_start;
    if active_tok {
        Ok((b"", Token::Word(input[tok_start..tok_end].to_vec())))
    } else {
        Err(nom::Err::Error(
            nom::error::Error::new(b"", ErrorKind::Fail)
        ))
    }
}

// fn word(input: &[u8]) -> Vec<u8> {
    
// }

// fn op(input: &[u8]) -> Vec<u8> {

// }

// test_token!(test_end, "foo", "foo", "");
// test_token!(test_space, "foo bar", "foo", "bar");
// test_token!(test_op, "foo|bar", "foo", "|bar");
// test_token!(test_op2, "|bar", "|", "bar");
// test_token!(test_op3, "foo&bar", "foo", "&bar");
// test_token!(test_op4, "&bar", "&", "bar");
// test_token!(test_op5, "foo;bar", "foo", ";bar");
// test_token!(test_op6, ";bar", ";", "bar");
// test_token!(test_op7, "foo&&bar", "foo", "&&bar");
// test_token!(test_op8, "&&bar", "&&", "bar");
// test_token!(test_op9, "foo||bar", "foo", "||bar");
// test_token!(test_op10, "||bar", "||", "bar");
// test_token!(test_op11, "foo || bar", "foo", "|| bar");
// test_token!(test_op12, "|| bar", "||", " bar");
// test_token!(test_newline, "foo\nbar", "foo\n", "bar");
// test_token!(test_newline2, "\n\nfoo", "\n", "\nfoo");
// test_token!(test_comment, "#foo\nbar", "\n", "bar");
// test_token!(test_comment2, "foo#bar\n", "foo#bar\n", "");


// mod tests {
//     use nom::IResult;
//     use nom::error;
//     use super::raw_token;
//     use std::str;

//     macro_rules! test_token {
//         ( $test_name:ident, $test_string:expr, $expected_tok:expr, $expected_remain:expr ) => {
//             #[test]
//             fn $test_name() {
//                 let test_string = $test_string.as_bytes().to_vec();
//                 let (remaining, tok) = raw_token(&test_string).unwrap();
//                 let remaining = str::from_utf8(&remaining).unwrap();
//                 let tok = str::from_utf8(&tok).unwrap();
//                 assert_eq!(tok, $expected_tok);
//                 assert_eq!(remaining, $expected_remain);
//             }
//         }
//     }

//     test_token!(test_end, "foo", "foo", "");
//     test_token!(test_space, "foo bar", "foo", "bar");
//     test_token!(test_op, "foo|bar", "foo", "|bar");
//     test_token!(test_op2, "|bar", "|", "bar");
//     test_token!(test_op3, "foo&bar", "foo", "&bar");
//     test_token!(test_op4, "&bar", "&", "bar");
//     test_token!(test_op5, "foo;bar", "foo", ";bar");
//     test_token!(test_op6, ";bar", ";", "bar");
//     test_token!(test_op7, "foo&&bar", "foo", "&&bar");
//     test_token!(test_op8, "&&bar", "&&", "bar");
//     test_token!(test_op9, "foo||bar", "foo", "||bar");
//     test_token!(test_op10, "||bar", "||", "bar");
//     test_token!(test_op11, "foo || bar", "foo", "|| bar");
//     test_token!(test_op12, "|| bar", "||", " bar");
//     test_token!(test_newline, "foo\nbar", "foo\n", "bar");
//     test_token!(test_newline2, "\n\nfoo", "\n", "\nfoo");
//     test_token!(test_comment, "#foo\nbar", "\n", "bar");
//     test_token!(test_comment2, "foo#bar\n", "foo#bar\n", "");

//     #[test]
//     fn test_empty() {
//         let test_string = b"";
//         let blah = b"".as_ref();
//         let expected: IResult<&[u8], &[u8], error::Error<&[u8]>> = Err(
//             nom::Err::Error(error::Error::new(blah, error::ErrorKind::Fail))
//         );
//         let actual_result = raw_token(test_string);
//         assert_eq!(actual_result, expected);
//     }
// }