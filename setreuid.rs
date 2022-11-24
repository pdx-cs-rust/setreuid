use std::fs::OpenOptions;
use std::io::Write;
use std::process::exit;

use cvt::cvt;
use libc::{uid_t, setreuid, getresuid};

fn fail<E: std::fmt::Display>(name: &str, e: E) -> ! {
    eprintln!("{name}: {e}");
    exit(1);
}

macro_rules! check {
    ($name:expr, $result:expr) => {
        cvt($result).unwrap_or_else(|e| fail($name, e))
    };
}

fn show_uids() -> (uid_t, uid_t, uid_t) {
    let mut ruid = 0;
    let mut euid = 0;
    let mut suid = 0;
    let result = unsafe { getresuid(&mut ruid, &mut euid, &mut suid) };
    check!("getresuid", result);
    println!("r={ruid}, e={euid}, s={suid}");
    (ruid, euid, suid)
}


fn main() {
    let (ruid, euid, _) = show_uids();

    let mut testfile = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("/tmp/testfile")
        .unwrap_or_else(|e| fail("open(1)", e));
    testfile.write_all(b"hello\n").unwrap_or_else(|e| fail("write_all(1)", e));
    drop(testfile);

    let result = unsafe { setreuid(euid, ruid) };
    check!("setreuid(1)", result);
    show_uids();

    let result = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/tmp/testfile");
    match result {
        Err(e) => println!("open(2): expected failure: {e}"),
        Ok(_) => {
            eprintln!("open(2): unexpected success");
            exit(1);
        }
    }

    let result = unsafe { setreuid(ruid, euid) };
    check!("setreuid(2)", result);
    show_uids();

    let mut testfile = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/tmp/testfile")
        .unwrap_or_else(|e| fail("open(3)", e));
    testfile.write_all(b"world\n").unwrap_or_else(|e| fail("write_all(2)", e));
}
