use anyhow::Result;

use clap::Parser;

use log::LevelFilter::{Debug, Info};
use log::{error, info};

use crate::cli::Cli;

mod cli;
mod master;
mod read;
mod scan_addr;
mod write;
// use tokio::time::sleep;

#[tokio::main]
pub async fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    match cli.log() {
        cli::LogLevel::Debug => custom_utils::logger::logger_stdout(Debug).log_to_stdout(),
        cli::LogLevel::Info => custom_utils::logger::logger_stdout(Info).log_to_stdout(),
    };
    info!("{:?}", cli);
    match cli {
        Cli::Read(read) => loop {
            match read.action().await {
                Ok(res) => {
                    info!(
                        "ack [str]: \t\t[{}]",
                        String::from_utf8_lossy(res.as_slice())
                    );
                    info!("ack [hex]: \t\t[{}]", pretty_hex::simple_hex(&res));
                }
                Err(err) => {
                    error!("err: {}", err);
                    break;
                }
            }
        },
        Cli::Write(write) => match write.action().await {
            Ok(res) => {
                info!("ack [str]: \t[{}]", String::from_utf8_lossy(res.as_slice()));
                info!("ack [hex]: \t[{}]", pretty_hex::simple_hex(&res));
            }
            Err(err) => error!("err: {}", err),
        },
        Cli::ScanAddr(config) => {
            if let Err(e) = config.action().await {
                error!("scan fail: {:?}", e);
            } else {
                info!("scan complete");
            }
        }
        Cli::Master(config) => {
            if let Err(e) = config.action().await {
                error!("scan fail: {:?}", e);
            } else {
                info!("scan complete");
            }
        }
    }
    Ok(())
}
