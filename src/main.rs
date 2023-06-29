use anyhow::Result;

use clap::Parser;

use log::LevelFilter::{Debug, Info};
use log::{error, info};

use crate::cli::Cli;

mod cli;
mod read;
mod write;
// use tokio::time::sleep;

#[tokio::main]
pub async fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    let _ = match cli.log() {
        cli::LogLevel::Debug => custom_utils::logger::logger_stdout(Debug),
        cli::LogLevel::Info => custom_utils::logger::logger_stdout(Info),
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
                Err(err) => error!("err: {}", err),
            }
        },
        Cli::Write(write) => match write.action().await {
            Ok(res) => {
                info!(
                    "ack [str]: \t\t[{}]",
                    String::from_utf8_lossy(res.as_slice())
                );
                info!("ack [hex]: \t\t[{}]", pretty_hex::simple_hex(&res));
            }
            Err(err) => error!("err: {}", err),
        },
    }
    // let (data, ending, builder) = cli.to_param()?;
    // match _collect_data_origin_by_arg(data, ending, builder).await {
    //     Ok(res) => {
    //         info!(
    //             "ack [str]: \t\t[{}]",
    //             String::from_utf8_lossy(res.as_slice())
    //         );
    //         info!("ack [hex]: \t\t[{}]", pretty_hex::simple_hex(&res));
    //     }
    //     Err(err) => error!("err: {}", err),
    // }
    Ok(())
}
