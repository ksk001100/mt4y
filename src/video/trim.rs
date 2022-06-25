use crate::utils::error_exit;
use seahorse::{Command, Context, Flag, FlagType};
use std::process::{Command as Exec, Stdio};

pub fn command() -> Command {
    Command::new("trim")
        .alias("tr")
        .usage(format!(
            "{} video trim [input file name] --start [start sec] --end [end sec] -o [output file name]",
            env!("CARGO_PKG_NAME")
        ))
        .description("Trims video at a specified time")
        .flag(
            Flag::new("start", FlagType::String)
                .alias("s")
                .description("Trim start seconds"),
        )
        .flag(
            Flag::new("end", FlagType::String)
                .alias("e")
                .description("Trim end seconds"),
        )
        .flag(
            Flag::new("output", FlagType::String)
                .alias("o")
                .description("Output file name"),
        )
        .action(action)
}

fn action(c: &Context) {
    let start = c.string_flag("start");
    let end = c.string_flag("end");

    if start.is_err() || end.is_err() || c.args.len() != 1 {
        error_exit();
    }

    let start = start.unwrap();
    let end = end.unwrap();
    let output = c
        .string_flag("output")
        .unwrap_or_else(|_| "output.mp4".to_string());

    Exec::new("ffmpeg")
        .arg("-ss")
        .arg(start)
        .arg("-to")
        .arg(end)
        .arg("-i")
        .arg(&c.args[0])
        .arg("-c")
        .arg("copy")
        .arg(output)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
