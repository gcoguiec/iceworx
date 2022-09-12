use miette::Diagnostic;
use thiserror::Error;

use crate::OpCode;

#[derive(Error, Diagnostic, Debug)]
pub enum IceError {
    #[error("Device '{0}' does not exist.")]
    InvalidDevice(String),
    #[error("File '{0}' does not exist.")]
    InvalidFile(String),
    #[error("Invalid offset '{0}'.")]
    InvalidOffset(usize),
    #[error("No response from '{0}' command.")]
    NoResponse(OpCode),
    #[error("Unknown Response.")]
    UnknownResponse,
    #[error("{0}, failed at {1:x}({2}), '{3:x}' expected, '{4:x}' read.")]
    VerificationError(OpCode, usize, u8, u8, u8)
}
