use std::env;
use std::ffi::CString;
use std::process::exit;

use nix::unistd::execvp;

use close_fds::close_open_fds;

const PROGNAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn usage() -> ! {
    eprintln!(
        r#"{} {}
usage: <FD> <COMMAND> <...>"#,
        PROGNAME, VERSION,
    );
    exit(2);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        usage()
    }

    let fd: i32 = args[0].parse()?;

    let argv: Vec<_> = args[1..]
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();

    unsafe {
        close_open_fds(fd, &[]);
    }

    execvp(&argv[0], &argv)?;

    unreachable!()
}
