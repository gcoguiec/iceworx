use async_trait::async_trait;
use iceworx::serial::Serial;
use tabled::{Style, Table};

use crate::{
    log::log,
    task::{Args, Command, Task, TaskResult}
};

pub struct BoardsTask;

#[async_trait]
impl Task for BoardsTask {
    fn command() -> Command {
        Command::new("boards").about("List all available iceFUN/iceWerx boards")
    }

    async fn run_task(_args: &Args) -> TaskResult {
        let ports = Serial::available_ports();
        log::info!(
            "{}",
            Table::new(iceworx::available_boards(ports))
                .with(Style::modern())
                .to_string()
        );

        Ok(())
    }
}
