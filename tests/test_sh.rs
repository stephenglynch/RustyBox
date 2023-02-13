use std::io::Write;
use std::process::Stdio;
use std::str;

mod common;

macro_rules! check_output {
    ( $test_name:ident, $command:expr, $args:expr, $expected:expr) => {
        #[test]
        fn $test_name() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = common::get_cmd("sh");
        
            let mut child = cmd
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            
            let child_stdin = child.stdin.as_mut().unwrap();
            child_stdin.write_all(concat!($command, " ", $args, "\n").as_bytes())?;
            drop(child_stdin);
        
            let output = child.wait_with_output()?;
            
            assert_eq!($expected, str::from_utf8(&output.stdout)?);
        
            Ok(())
        }
    }
}

check_output!(sh_echo1, "echo", "fooword", "fooword\n");

check_output!(sh_echo2, "echo", "foo word", "foo word\n");
