use std::ffi::OsStr;

use clap::{crate_authors, crate_version, App, AppSettings, Arg};

pub mod log;
pub mod task;

use crate::{
    log::{logger_init, Format, Level},
    task::{BoardsTask, FlashTask, Task}
};

#[tokio::main]
async fn main() -> miette::Result<()> {
    let exe = std::env::current_exe().unwrap_or_default();
    let name = exe
        .as_path()
        .file_name()
        .unwrap_or_else(|| OsStr::new("iceworx"))
        .to_str()
        .unwrap_or_default();

    let matches = App::new(name)
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("trace").long("trace").help("Enables tracing"))
        .subcommand(BoardsTask::command())
        .subcommand(FlashTask::command())
        .get_matches();

    let logger_args = if matches.is_present("trace") {
        (Level::TRACE, Format::Trace)
    } else {
        (Level::INFO, Format::Default)
    };

    logger_init(logger_args.0, logger_args.1)?;

    match matches.subcommand() {
        Some(("boards", matches)) => {
            BoardsTask::run_task(matches).await?;
        }
        Some(("flash", matches)) => {
            FlashTask::run_task(matches).await?;
        }
        _ => {}
    }

    Ok(())
}
