use crate::cli::{DataBits, FlowControl, LogLevel, Parity, StopBits};
use anyhow::Result;
use bytes::BytesMut;
use clap::Parser;
use log::debug;
use tokio::io::AsyncReadExt;
use tokio_serial::SerialStream;

#[derive(Parser, Debug)]
pub struct Reader {
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

impl Reader {
    pub async fn action(&self) -> Result<Vec<u8>> {
        let builder = tokio_serial::new(self.path.clone(), self.baud_rate)
            .data_bits(self.data_bits.into())
            .parity(self.parity.into())
            .flow_control(self.flow_control.into())
            .stop_bits(self.stop_bits.into());

        #[allow(unused_mut)]
        let mut stream = SerialStream::open(&builder)?;

        #[cfg(unix)]
        stream.set_exclusive(true)?;
        debug!("打开串口成功");
        _collect_data_origin_by_arg(stream).await
    }
}

async fn _collect_data_origin_by_arg(mut device: SerialStream) -> Result<Vec<u8>> {
    let mut datas = BytesMut::new();
    let mut buf = BytesMut::with_capacity(1024);
    loop {
        let Ok(bytes_read) = device
            .read_buf(&mut buf).await else {
            return Ok(datas.to_vec());
        };
        debug!("read {} bytes", bytes_read);
        datas.extend_from_slice(&buf[0..bytes_read]);
    }
}
