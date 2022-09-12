use async_trait::async_trait;

mod boards_task;
mod flash_task;

pub use boards_task::BoardsTask;
pub use flash_task::FlashTask;

pub type Arg<'help> = clap::Arg<'help>;
pub type Args = clap::ArgMatches;
pub type Command = clap::Command<'static>;
pub type CommandSettings = clap::AppSettings;
pub type TaskResult = miette::Result<()>;

#[async_trait]
pub trait Task: Send {
    fn command() -> Command;
    async fn run_task(args: &Args) -> TaskResult;
}
