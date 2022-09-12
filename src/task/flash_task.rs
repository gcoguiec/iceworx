use async_trait::async_trait;
use iceworx::{
    bitstream::{Bitstream, BitstreamOptions},
    error::IceError,
    flash,
    release_fpga,
    reset_fpga,
    serial::Serial,
    verify_flash,
    FlashHooks,
    Offset,
    VerifyHooks
};
use indicatif::ProgressBar;

use crate::task::{Arg, Args, Command, CommandSettings, Task, TaskResult};

pub struct FlashTask;

#[async_trait]
impl Task for FlashTask {
    fn command() -> Command {
        Command::new("flash")
            .about("Flash an iceFUN/iceWerx board")
            .setting(CommandSettings::ArgRequiredElseHelp)
            .arg(
                Arg::with_name("file")
                    .required(true)
                    .help("Specify the binary file to flash the board with")
            )
            .arg(
                Arg::with_name("device")
                    .short('d')
                    .long("device")
                    .required(true)
                    .value_name("device")
                    .takes_value(true)
                    .help("Specify the device to flash (e.g. /dev/ttyACM0)")
            )
            .arg(
                Arg::with_name("offset")
                    .short('o')
                    .long("offset")
                    .value_name("offset address")
                    .help("Specify an address offset")
                    .value_parser(clap::value_parser!(Offset))
            )
            .arg(
                Arg::with_name("noverify")
                    .long("no-verify")
                    .help("Skip flash verification")
                    .value_parser(clap::value_parser!(bool))
            )
    }

    async fn run_task(args: &Args) -> TaskResult {
        let device = args.get_one::<String>("device").unwrap();
        let filename = args.get_one::<String>("file").unwrap();
        let offset = args.get_one::<Offset>("offset").unwrap_or(&Offset(0));
        let no_verify = args.is_present("noverify");

        let mut serial = Serial::connect(device)?;
        let bitstream =
            Bitstream::new(filename, BitstreamOptions { offset: offset.0 })?;
        log::trace!("{}", bitstream);

        if bitstream.offset > bitstream.filesize {
            return Err(IceError::InvalidOffset(bitstream.offset).into())
        }

        reset_fpga(&mut serial)?;

        let bar = ProgressBar::new(bitstream.filesize as u64);
        log::info!(
            "Flashing '{}' on {}:",
            bitstream.filepath.file_name().unwrap().to_string_lossy(),
            device
        );

        flash(&mut serial, &bitstream, FlashHooks {
            on_page_written: Box::new(
                move |start_addr: usize, is_last: bool| {
                    if is_last {
                        bar.finish();
                    } else {
                        bar.set_position(start_addr as u64);
                    }
                }
            )
        })?;

        if !no_verify {
            let bar = ProgressBar::new(bitstream.filesize as u64);
            log::info!("Verifying flash:");
            verify_flash(&mut serial, &bitstream, VerifyHooks {
                on_page_verified: Box::new(
                    move |start_addr: usize, is_last: bool| {
                        if is_last {
                            bar.finish();
                        } else {
                            bar.set_position(start_addr as u64)
                        }
                    }
                )
            })?;
        }

        release_fpga(&mut serial)?;
        log::info!("Done.");

        Ok(())
    }
}
