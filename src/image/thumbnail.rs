use crate::utils::error_exit;
use seahorse::{Command, Context, Flag, FlagType};
use std::process::{Command as Exec, Stdio};

pub fn command() -> Command {
    Command::new("thumbnail")
        .alias("t")
        .usage(format!(
            "{} image thumbnail [input file] [flags]",
            env!("CARGO_PKG_NAME")
        ))
        .description("Generate thumbnail")
        .flag(
            Flag::new("time", FlagType::Int)
                .alias("t")
                .description("Time specification for thumbnail images (in seconds)"),
        )
        .flag(
            Flag::new("output", FlagType::String)
                .alias("o")
                .description("Output file name"),
        )
        .action(action)
}

fn action(c: &Context) {
    if c.args.len() != 1 {
        error_exit();
    }

    let output = c
        .string_flag("output")
        .unwrap_or_else(|_| "output.jpg".to_string());

    let time = c.int_flag("time").unwrap_or_else(|_| 1);

    Exec::new("ffmpeg")
        .arg("-i")
        .arg(&c.args[0])
        .arg("-ss")
        .arg(&time.to_string())
        .arg("-vframes")
        .arg("1")
        .arg("-f")
        .arg("image2")
        .arg(&output)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
