use std::time::Duration;

use miette::{IntoDiagnostic, Result};
use serialport::{
    DataBits,
    Parity,
    SerialPort,
    SerialPortInfo,
    SerialPortType,
    StopBits
};

pub struct Serial;

impl Serial {
    pub fn available_ports() -> Vec<SerialPortInfo> {
        serialport::available_ports()
            .unwrap_or_default()
            .into_iter()
            .filter(|port| {
                matches!(
                    &port.port_type,
                    SerialPortType::UsbPort(..) | SerialPortType::Unknown
                )
            })
            .collect()
    }

    pub fn connect(device: &str) -> Result<Box<dyn SerialPort>> {
        serialport::new(device, 115_200)
            .timeout(Duration::from_millis(1_000))
            .parity(Parity::None)
            .stop_bits(StopBits::Two)
            .data_bits(DataBits::Eight)
            .open()
            .into_diagnostic()
    }
}
