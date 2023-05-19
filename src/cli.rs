use std::time::Duration;

use clap::{Parser, ValueEnum};
use serialport::SerialPortBuilder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    path: String,
    data: String,
    #[arg(default_value_t = 9600, short)]
    baud_rate: u32,
    #[arg(value_enum, default_value_t= DataBits::Eight, short)]
    data_bits: DataBits,
    #[arg(value_enum, default_value_t= Parity::None, short)]
    parity: Parity,
    #[arg(value_enum, default_value_t= StopBits::One, short)]
    stop_bits: StopBits,
    #[arg(value_enum, default_value_t= FlowControl::None, short)]
    flow_control: FlowControl,
    #[arg(default_value_t = 500, short)]
    timeout: u64,
    #[arg(value_enum, default_value_t= Ending::None, short)]
    ending: Ending,
    #[arg(value_enum, default_value_t= LogLevel::Info, short)]
    pub log: LogLevel,
}

impl Cli {
    pub fn to_param(self) -> (String, Ending, SerialPortBuilder) {
        let buidler = serialport::new(self.path, self.baud_rate)
            .data_bits(self.data_bits.into())
            .parity(self.parity.into())
            .flow_control(self.flow_control.into())
            .stop_bits(self.stop_bits.into())
            .timeout(Duration::from_millis(self.timeout));
        (self.data, self.ending, buidler)
    }
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
