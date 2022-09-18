use assert_cmd::prelude::*;

mod common;

macro_rules! test_basename {
    ( $test_name:ident, $test_args:expr, $expected:expr ) => {
        #[test]
        fn $test_name() -> Result<(), Box<dyn std::error::Error>> {
            let mut cmd = common::get_cmd("basename");

            // Ensure null string is supplied
            if $test_args == "" {
                cmd.arg("");
            }

            for arg in $test_args.split_whitespace() {
                cmd.arg(arg);
            }
            cmd.assert()
                .success()
                .stdout($expected);

            Ok(())
        }
    }
}


// Basic usage
test_basename!(basic_1, "foo/bar", "bar\n");

// Null string
test_basename!(null_str, "", ".\n");

// Removal of extra /'s
test_basename!(slashes_only, "///////", "/\n");
test_basename!(trailing_slashes, "a//////", "a\n");
test_basename!(combined_slashes, "/////a///b///c///d/////", "d\n");

// Standard suffix behavior.
test_basename!(standard_suffix, "a/b/c/d.suffix .suffix", "d\n");

// A suffix cannot be the entire result.
test_basename!(suffix_result, ".txt .txt", ".txt\n");

// Deal with suffix appearing in the filename
test_basename!(reappearing_suffix_1, "a.txt.txt .txt", "a.txt\n");
test_basename!(reappearing_suffix_2, "a.txt.old .txt", "a.txt.old\n");

// A suffix should be a real suffix, only a the end.
test_basename!(invalid_suffix, "isthisasuffix? suffix", "isthisasuffix?\n");

// Zero-length suffix
test_basename!(zero_length_suffix, "a/b/c ''", "c\n");
