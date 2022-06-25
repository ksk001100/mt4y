use crate::utils::error_exit;
use seahorse::{Command, Context, Flag, FlagType};
use std::process::{Command as Exec, Stdio};

pub fn command() -> Command {
    Command::new("rotation")
        .alias("r")
        .usage(format!(
            "{} video rotation [input file] [flags]",
            env!("CARGO_PKG_NAME")
        ))
        .description("Rotation video")
        .flag(
            Flag::new("left", FlagType::Bool)
                .alias("l")
                .description("Rotate to the left"),
        )
        .flag(
            Flag::new("right", FlagType::Bool)
                .alias("r")
                .description("Rotate to the right"),
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
        .unwrap_or_else(|_| "output.mp4".to_string());

    let r = match (c.bool_flag("left"), c.bool_flag("right")) {
        (true, false) => 2,
        (false, true) => 1,
        (_, _) => error_exit(),
    };

    Exec::new("ffmpeg")
        .arg("-i")
        .arg(&c.args[0])
        .arg("-vf")
        .arg(&format!("transpose={}", r))
        .arg(&output)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
