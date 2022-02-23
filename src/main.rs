use std::env;
use std::process::{exit, Command as Exec, Stdio};

use seahorse::{App, Command, Flag, FlagType};

fn main() {
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage(format!("{} [command] [flags]", env!("CARGO_PKG_NAME")))
        .version(env!("CARGO_PKG_VERSION"))
        .command(rotation_command())
        .command(thumbnail_command())
        .run(env::args().collect());
}

fn rotation_command() -> Command {
    Command::new("rotation")
        .alias("r")
        .usage(format!(
            "{} rotation [input file] [flags]",
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
        .action(|c| {
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
        })
}

fn thumbnail_command() -> Command {
    Command::new("thumbnail")
        .alias("t")
        .usage(format!(
            "{} thumbnail [input file] [flags]",
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
        .action(|c| {
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
        })
}

fn error_exit() -> ! {
    eprint!("Error...");
    exit(1)
}
