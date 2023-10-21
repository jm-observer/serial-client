use crate::cli::{DataBits, LogLevel, Parity, StopBits};
use anyhow::{Context, Result};
use bytes::BytesMut;
use clap::Parser;
use crc::{Crc, CRC_16_MODBUS};
use log::{debug, info};
use serialport::SerialPortBuilder;
use std::time::Duration;

pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_MODBUS);

#[derive(Parser, Debug)]
pub struct ScanAddr {
    path: String,
    #[arg(default_value_t = 9600, short)]
    baud_rate: u32,
    #[arg(value_enum, default_value_t= DataBits::Eight, short)]
    data_bits: DataBits,
    #[arg(value_enum, default_value_t= Parity::None, short)]
    parity: Parity,
    #[arg(value_enum, default_value_t= StopBits::One, short)]
    stop_bits: StopBits,
    #[arg(default_value_t = 500, short)]
    timeout: u64,
    #[arg(default_value_t = 3, short)]
    func_code: u8,
    #[arg(default_value_t = 0, short = 'a')]
    register_addr: u16,
    #[arg(default_value_t = 1, short = 'r')]
    register_len: u16,
    #[arg(default_value_t = false, short)]
    with_parity: bool,
    #[arg(value_enum, default_value_t= LogLevel::Info, short)]
    pub log: LogLevel,
}

impl ScanAddr {
    pub async fn action(&self) -> Result<()> {
        let mut parities = Vec::with_capacity(3);
        if self.with_parity {
            parities.push(Parity::None);
            parities.push(Parity::Even);
            parities.push(Parity::Odd);
        } else {
            parities.push(self.parity);
        }
        for parity in parities {
            let builder = serialport::new(self.path.clone(), self.baud_rate)
                .data_bits(self.data_bits.into())
                .parity(parity.into())
                .stop_bits(self.stop_bits.into())
                .timeout(Duration::from_millis(self.timeout));
            if self._collect_data_origin_by_arg(builder, parity).await? {
                break;
            }
        }
        Ok(())
    }

    async fn _collect_data_origin_by_arg(
        &self,
        builder: SerialPortBuilder,
        parity: Parity,
    ) -> Result<bool> {
        let mut device = builder.open().context("打开串口失败")?;
        let register_addr = self.register_addr.to_be_bytes();
        let register_len = self.register_len.to_be_bytes();
        let mut data = [
            0u8,
            self.func_code,
            register_addr[0],
            register_addr[1],
            register_len[0],
            register_len[1],
            0u8,
            0u8,
        ];
        let mut found = false;
        for i in 1..u8::MAX {
            data[0] = i;
            let check_num = X25.checksum(&data[0..=5]).to_le_bytes();
            data[6] = check_num[0];
            data[7] = check_num[1];
            debug!("{:x?}", data);
            device.write_all(data.as_slice())?;
            let mut datas = BytesMut::new();
            let mut buf = vec![0; 1024];
            loop {
                let Ok(bytes_read) = device.read(&mut buf) else {
                    if !datas.is_empty() {
                        info!("addr({:02x?}) parity({:?}), write: {}, response: {}", i, parity, pretty_hex::simple_hex(&data), pretty_hex::simple_hex(&datas.freeze()));
                        found = true;
                        break;
                    } else {
                        info!("addr({}) parity({:?}): timeout", i, parity);
                        break;
                    }
                };
                info!("read {:3 } bytes", bytes_read);
                datas.extend_from_slice(&buf[0..bytes_read]);
            }
            if found {
                break;
            }
        }
        Ok(found)
    }
}
