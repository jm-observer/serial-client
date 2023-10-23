use crate::read::Reader;
use crate::scan_addr::*;
use crate::slave::Slave;
use crate::write::Writer;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub enum Cli {
    Read(Reader),
    Write(Writer),
    ScanAddr(ScanAddr),
    Slave(Slave),
}

impl Cli {
    pub fn log(&self) -> LogLevel {
        match self {
            Cli::Read(read) => read.log,
            Cli::Write(write) => write.log,
            Cli::ScanAddr(config) => config.log,
            Cli::Slave(config) => config.log,
        }
    }
}

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum DataType {
    Str,
    Hex,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum LogLevel {
    Debug,
    Info,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum Ending {
    /// not ending to append data
    None,
    /// \r
    R,
    /// \r\n
    Rn,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum StopBits {
    /// One stop bit.
    One,
    /// Two stop bits.
    Two,
}

impl Into<serialport::StopBits> for StopBits {
    fn into(self) -> serialport::StopBits {
        match self {
            StopBits::One => serialport::StopBits::One,
            StopBits::Two => serialport::StopBits::Two,
        }
    }
}

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum Parity {
    /// No parity bit.
    None,
    /// Parity bit sets odd number of 1 bits.
    Odd,
    /// Parity bit sets even number of 1 bits.
    Even,
}

impl Into<serialport::Parity> for Parity {
    fn into(self) -> serialport::Parity {
        match self {
            Parity::None => serialport::Parity::None,
            Parity::Odd => serialport::Parity::Odd,
            Parity::Even => serialport::Parity::Even,
        }
    }
}

/// Number of bits per character
#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum DataBits {
    /// 5 bits per character
    Five,
    /// 6 bits per character
    Six,
    /// 7 bits per character
    Seven,
    /// 8 bits per character
    Eight,
}

impl Into<serialport::DataBits> for DataBits {
    fn into(self) -> serialport::DataBits {
        match self {
            DataBits::Five => serialport::DataBits::Five,
            DataBits::Six => serialport::DataBits::Six,
            DataBits::Seven => serialport::DataBits::Seven,
            DataBits::Eight => serialport::DataBits::Eight,
        }
    }
}

#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum FlowControl {
    /// No flow control.
    None,
    /// Flow control using XON/XOFF bytes.
    Software,
    /// Flow control using RTS/CTS signals.
    Hardware,
}

impl Into<serialport::FlowControl> for FlowControl {
    fn into(self) -> serialport::FlowControl {
        match self {
            FlowControl::None => serialport::FlowControl::None,
            FlowControl::Software => serialport::FlowControl::Software,
            FlowControl::Hardware => serialport::FlowControl::Hardware,
        }
    }
}
