use std::io::Read;

use miette::{IntoDiagnostic, Result};
use num_traits::Zero;
use serialport::{SerialPort, SerialPortInfo, SerialPortType};
use strum_macros::Display;
use tabled::Tabled;

pub mod bitstream;
pub mod error;
pub mod parse;
pub mod serial;

use crate::{
    bitstream::{Bitstream, Chunk},
    error::IceError,
    parse::OffsetParser
};

pub const PRODUCT_NAME: &str = "iceFUN";
pub const FLASHSIZE: usize = 0x0100000;
pub const CHUNK_SIZE: usize = 0x100;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Display)]
#[repr(u8)]
pub enum OpCode {
    Done = 0xB0,
    GetVersion,
    ResetFpga,
    EraseChip,
    Erase64K,
    ProgPage,
    ReadPage,
    VerifyPage,
    GetCDone,
    ReleaseFpga
}

#[derive(Tabled)]
pub struct Board {
    #[tabled(rename = "Vendor")]
    pub vendor: String,
    #[tabled(rename = "Board Name")]
    pub board: String,
    #[tabled(rename = "Device Path")]
    pub device: String
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Offset(pub usize);

impl clap::builder::ValueParserFactory for Offset {
    type Parser = OffsetParser;

    fn value_parser() -> Self::Parser {
        OffsetParser
    }
}

pub struct FlashHooks {
    pub on_page_written: Box<dyn Fn(usize, bool)>
}

pub struct VerifyHooks {
    pub on_page_verified: Box<dyn Fn(usize, bool)>
}

pub fn available_boards(ports: Vec<SerialPortInfo>) -> Vec<Board> {
    let mut boards = vec![];
    ports
        .iter()
        .filter(|port| {
            matches!(&port.port_type, SerialPortType::UsbPort(device)
                if device.product == Some(String::from(PRODUCT_NAME))
                    || device.product.is_none())
        })
        .for_each(|port| {
            if let SerialPortType::UsbPort(device) = &port.port_type {
                boards.push(Board {
                    vendor: device.manufacturer.as_ref().unwrap().to_string(),
                    board: device.product.as_ref().unwrap().to_string(),
                    device: port.port_name.to_owned()
                });
            }
        });
    boards
}

pub fn reset_fpga(serial: &mut Box<dyn SerialPort>) -> Result<()> {
    serial.write(&[OpCode::ResetFpga as u8]).into_diagnostic()?;

    let mut buf = [0u8; 3];
    let size = serial.read(&mut buf).into_diagnostic()?;
    if size.is_zero() {
        return Err(IceError::NoResponse(OpCode::ResetFpga).into())
    }
    log::trace!(
        "{}, flash ID = {:x} {:x} {:x}",
        OpCode::ResetFpga,
        buf[0],
        buf[1],
        buf[2]
    );

    Ok(())
}

pub fn release_fpga(serial: &mut Box<dyn SerialPort>) -> Result<()> {
    serial.write(&[OpCode::ReleaseFpga as u8]).into_diagnostic()?;

    let mut buffer = [0u8; 1];
    let size = serial.read(&mut buffer).into_diagnostic()?;
    if size.is_zero() {
        return Err(IceError::NoResponse(OpCode::ReleaseFpga).into())
    }
    log::trace!("{}, response = {:x}", OpCode::ReleaseFpga, buffer[0]);

    Ok(())
}

pub fn get_version(serial: &mut Box<dyn SerialPort>) -> Result<u8> {
    serial.write(&[OpCode::GetVersion as u8]).into_diagnostic()?;

    let mut buffer = [0u8; 2];
    let size = serial.read(&mut buffer).into_diagnostic()?;
    if size.is_zero() {
        return Err(IceError::NoResponse(OpCode::GetVersion).into())
    }

    if buffer[0] != 0x26 {
        return Err(IceError::UnknownResponse.into())
    }

    Ok(buffer[1])
}

pub fn erase_page(serial: &mut Box<dyn SerialPort>, page: u8) -> Result<()> {
    serial.write(&[OpCode::Erase64K as u8, page]).into_diagnostic()?;

    let mut buf = [0u8; 1];
    if serial.read(&mut buf).into_diagnostic()?.is_zero() {
        return Err(IceError::NoResponse(OpCode::Erase64K).into())
    }
    log::trace!("{}, sector = {:x}0000", OpCode::Erase64K, page);

    Ok(())
}

pub fn verify_flash(
    serial: &mut Box<dyn SerialPort>, bitstream: &Bitstream, hooks: VerifyHooks
) -> Result<()> {
    for Chunk(start_addr, range, is_last) in bitstream.chunk_iter() {
        let mut payload: Vec<u8> = vec![
            OpCode::VerifyPage as u8,
            (start_addr >> 16) as u8,
            (start_addr >> 8) as u8,
            start_addr as u8,
        ];

        payload.extend(bitstream.buffer[range].iter());

        if is_last {
            let padding = CHUNK_SIZE - (bitstream.filesize - start_addr);
            payload.extend(vec![0u8; padding].iter());
        }

        serial.write(payload.as_slice()).into_diagnostic()?;
        let mut buf = [0u8; 4];
        let response = serial.read(&mut buf).into_diagnostic()?;
        if response.is_zero() {
            return Err(IceError::NoResponse(OpCode::VerifyPage).into())
        }

        log::trace!("{}, start_addr = {}", OpCode::VerifyPage, start_addr);
        if !buf[0].is_zero() {
            return Err(IceError::VerificationError(
                OpCode::VerifyPage,
                start_addr,
                buf[1],
                buf[2],
                buf[3]
            )
            .into())
        }

        (hooks.on_page_verified)(start_addr, is_last);
    }

    Ok(())
}

pub fn flash(
    serial: &mut Box<dyn SerialPort>, bitstream: &Bitstream, hooks: FlashHooks
) -> Result<()> {
    let last_page = ((bitstream.offset + bitstream.filesize) >> 16) + 1;
    for page in (bitstream.offset >> 16)..last_page {
        erase_page(serial, page as u8)?;
    }
    for Chunk(start_addr, range, is_last) in bitstream.chunk_iter() {
        let mut payload: Vec<u8> = vec![
            OpCode::ProgPage as u8,
            (start_addr >> 16) as u8,
            (start_addr >> 8) as u8,
            start_addr as u8,
        ];

        payload.extend(bitstream.buffer[range].iter());

        if is_last {
            let padding = CHUNK_SIZE - (bitstream.filesize - start_addr);
            payload.extend(vec![0u8; padding].iter());
        }

        serial.write(payload.as_slice()).into_diagnostic()?;
        let mut buf = [0u8; 4];
        let response = serial.read(&mut buf).into_diagnostic()?;
        log::trace!("{}, addr = {}", OpCode::ProgPage, start_addr);
        if response.is_zero() {
            return Err(IceError::NoResponse(OpCode::ProgPage).into())
        }

        (hooks.on_page_written)(start_addr, is_last);
    }

    Ok(())
}
