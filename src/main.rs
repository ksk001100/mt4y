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
        .command(trim_command())
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
        .alias("th")
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

fn trim_command() -> Command {
    Command::new("trim")
        .alias("tr")
        .usage(format!(
            "{} trim [input file name] --start [start sec] --end [end sec] -o [output file name]",
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
        .action(|c| {
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
        })
}

fn error_exit() -> ! {
    eprint!("Error...");
    exit(1)
}
