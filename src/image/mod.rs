mod thumbnail;
mod compress;


use seahorse::Command;

pub fn command() -> Command {
    Command::new("image")
        .alias("i")
        .usage(format!(
            "{} image [sub command] [...]",
            env!("CARGO_PKG_NAME")
        ))
        .description("Image command")
        .command(thumbnail::command())
        .command(compress::command())
}