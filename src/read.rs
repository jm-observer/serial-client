use crate::cli::{DataBits, FlowControl, LogLevel, Parity, StopBits};
use anyhow::{Context, Result};
use bytes::BytesMut;
use clap::Parser;
use log::{debug, info};
use serialport::SerialPortBuilder;
use std::time::Duration;

#[derive(Parser, Debug)]
pub struct Read {
    path: String,
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
    #[arg(value_enum, default_value_t= LogLevel::Info, short)]
    pub log: LogLevel,
    #[arg(default_value_t = 500, short)]
    timeout: u64,
}

impl Read {
    pub async fn action(&self) -> Result<Vec<u8>> {
        let buidler = serialport::new(self.path.clone(), self.baud_rate)
            .data_bits(self.data_bits.into())
            .parity(self.parity.into())
            .flow_control(self.flow_control.into())
            .stop_bits(self.stop_bits.into())
            .timeout(Duration::from_millis(self.timeout));
        _collect_data_origin_by_arg(buidler).await
    }
}

async fn _collect_data_origin_by_arg(builder: SerialPortBuilder) -> Result<Vec<u8>> {
    let mut device = builder.open().context("打开串口失败")?;
    info!("打开串口成功");

    #[cfg(unix)]
    stream
        .set_exclusive(true)
        .map_err(|_| Error::SerialPortSetExclusive(dev_path.clone()))?;

    let mut datas = BytesMut::new();
    let mut buf = vec![0; 1024];
    loop {
        let Ok(bytes_read) = device
            .read(&mut buf) else {
            return Ok(datas.to_vec());
        };
        debug!("read {} bytes", bytes_read);
        datas.extend_from_slice(&buf[0..bytes_read]);
    }
}
