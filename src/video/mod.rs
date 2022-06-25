mod rotation;
mod trim;

use seahorse::Command;

pub fn command() -> Command {
    Command::new("video")
        .alias("v")
        .usage(format!(
            "{} video [sub command] [...]",
            env!("CARGO_PKG_NAME")
        ))
        .description("Video command")
        .command(rotation::command())
        .command(trim::command())
}
