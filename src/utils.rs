use std::process::exit;

pub fn error_exit() -> ! {
    eprint!("Error...");
    exit(1)
}
