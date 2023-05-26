use anyhow::{Context, Result};
use bytes::BytesMut;
use clap::Parser;
use cli::{Cli, Ending};
use log::LevelFilter::{Debug, Info};
use log::{debug, error, info};
use serialport::SerialPortBuilder;

mod cli;
// use tokio::time::sleep;

#[tokio::main]
pub async fn main() -> Result<()> {
    let cli = Cli::parse();
    let _ = match cli.log {
        cli::LogLevel::Debug => custom_utils::logger::logger_stdout(Debug),
        cli::LogLevel::Info => custom_utils::logger::logger_stdout(Info),
    };
    custom_utils::logger::logger_stdout_debug();

    info!("{:?}", cli);
    let (data, ending, builder) = cli.to_param()?;
    match _collect_data_origin_by_arg(data, ending, builder).await {
        Ok(res) => {
            info!(
                "ack [str]: \t\t[{}]",
                String::from_utf8_lossy(res.as_slice())
            );
            info!("ack [hex]: \t\t[{}]", pretty_hex::simple_hex(&res));
        }
        Err(err) => error!("err: {}", err),
    }
    Ok(())
}
async fn _collect_data_origin_by_arg(
    data: Vec<u8>,
    ending: Ending,
    builder: SerialPortBuilder,
) -> Result<Vec<u8>> {
    let mut device = builder.open().context("打开串口失败")?;
    info!("打开串口成功，准备写入：{:x?}", data);

    device.write_all(data.as_slice())?;
    match ending {
        Ending::None => (),
        Ending::R => {
            device.write([b'\n'].as_ref())?;
        }
        Ending::Rn => {
            device.write([b'\r', b'\n'].as_ref())?;
        }
    }

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
