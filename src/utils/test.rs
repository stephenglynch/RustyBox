use std::ffi::{OsString, OsStr}; 
use std::os::unix::prelude::{FileTypeExt, MetadataExt};
use std::os::unix::prelude::OsStrExt;
use std::process::ExitCode;
use std::error::Error;
use std::fs;
use std::str;
use std::path::Path;
use std::io;
use std::vec::Vec;
use log::*;
use crate::safe_libc;
use libc;

enum FileTest {
    BlockDev,     /* -b */
    CharDev,      /* -c */
    Directory,    /* -d */
    Exist,        /* -e */
    File,         /* -f */
    GidSet,       /* -g */
    SymLink,      /* -hL */
    Fifo,         /* -p */
    Readable,     /* -r */
    Socket,       /* -S */
    GtZero,       /* -s */
    UidSet,       /* -u */
    Writable,     /* -w */
    Executable    /* -x */
}

#[derive(PartialEq)]
enum Mode {
    Read,
    Write,
    Execute
}

fn test_expr1(arg1: &[u8]) -> bool {
    if arg1.len() == 0 {
        false
    } else {
        true
    }
}

fn check_access(path: &Path, mode: Mode) -> io::Result<bool> {
    let euid = safe_libc::geteuid();

    let (user_bit, group_bit, other_bit) = match mode {
        Mode::Read => (libc::S_IRUSR, libc::S_IRGRP, libc::S_IROTH),
        Mode::Write => (libc::S_IWUSR, libc::S_IWGRP, libc::S_IWOTH),
        Mode::Execute => (libc::S_IXUSR, libc::S_IXGRP, libc::S_IXOTH),
    };

    let st = safe_libc::stat(path)?;

    // Check if root
    if euid == 0 {
        if mode == Mode::Execute {
            return Ok(st.st_mode & (user_bit & group_bit & other_bit) != 0)
        } else {
            return Ok(true)
        }
    }

    // Check if owner
    if euid == st.st_uid && (st.st_mode & user_bit) != 0 {
        return Ok(true)
    }

    // Check if part of groups
    let gid = safe_libc::getegid();
    let groups = safe_libc::getgroups();
    if groups.contains(&gid) && (st.st_mode & group_bit) != 0 {
        return Ok(true)
    }

    // Check if others can access
    if (st.st_mode & other_bit) != 0 {
        return Ok(true)
    }

    return Ok(false)
}

fn is_readable(path: &Path) -> io::Result<bool> {
    check_access(path, Mode::Read)
}

fn is_writeable(path: &Path) -> io::Result<bool> {
    check_access(path, Mode::Write)
}

fn is_excuteable(path: &Path) -> io::Result<bool> {
    check_access(path, Mode::Execute)
}

fn test_path(test_type: FileTest, path: &Path) -> io::Result<bool> {

    // Check path exists first
    if !path.exists() {
        return Ok(false)
    }

    let metadata = fs::metadata(path)?;
    let file_type = metadata.file_type();
    let mode = metadata.mode();

    let ret = match test_type {
        FileTest::BlockDev => file_type.is_block_device(),
        FileTest::CharDev => file_type.is_char_device(),
        FileTest::Directory => file_type.is_dir(),
        FileTest::Exist => true,
        FileTest::File => file_type.is_file(),
        FileTest::GidSet => (mode & libc::S_ISGID) != 0,
        FileTest::SymLink => file_type.is_symlink(),
        FileTest::Fifo => file_type.is_fifo(),
        FileTest::Readable => is_readable(path)?,
        FileTest::Socket => file_type.is_socket(),
        FileTest::GtZero => metadata.size() > 0,
        FileTest::UidSet => (mode & libc::S_ISGID) != 0,
        FileTest::Writable => is_writeable(path)?,
        FileTest::Executable => is_excuteable(path)?
    };

    Ok(ret)
}

fn test_expr2(arg1: &[u8], arg2: &[u8]) -> Result<bool, Box<dyn Error>> {
    let path = Path::new(OsStr::from_bytes(arg2));
    let ret = match arg1 {
        b"!" => !test_expr1(arg2),
        b"-b" => test_path(FileTest::BlockDev, path)?,
        b"-c" => test_path(FileTest::CharDev, path)?,
        b"-d" => test_path(FileTest::Directory, path)?,
        b"-e" => test_path(FileTest::Exist, path)?,
        b"-f" => test_path(FileTest::File, path)?,
        b"-g" => test_path(FileTest::GidSet, path)?,
        b"-h" => test_path(FileTest::SymLink, path)?,
        b"-L" => test_path(FileTest::SymLink, path)?,
        b"-n" => arg2.len() != 0,
        b"-p" => test_path(FileTest::Fifo, path)?,
        b"-r" => test_path(FileTest::Readable, path)?,
        b"-S" => test_path(FileTest::Socket, path)?,
        b"-s" => test_path(FileTest::GtZero, path)?,
        b"-t" => safe_libc::isatty(i32::from_str_radix(str::from_utf8(arg2)?, 10)?)?,
        b"-u" => test_path(FileTest::UidSet, path)?,
        b"-w" => test_path(FileTest::Writable, path)?,
        b"-x" => test_path(FileTest::Executable, path)?,
        b"-z" => arg2.len() == 0,
        _ => {
            error!("Unrecognised operator");
            false
        }
    };

    Ok(ret)
}

pub fn test_main(args: Vec<OsString>) -> Result<ExitCode, Box<dyn Error>> {

    let ret = if args.len() == 0 {
        false
    } else if args.len() == 1 {
        test_expr1(args[0].as_bytes())
    } else if args.len() == 2{
        test_expr2(args[0].as_bytes(), args[1].as_bytes())?
    } else {
        false
    };
    
    if ret {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}