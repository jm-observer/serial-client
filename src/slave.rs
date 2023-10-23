use crate::cli::{DataBits, FlowControl, LogLevel, Parity, StopBits};
use anyhow::Result;
use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use serialport::SerialPortBuilder;
use std::time::Duration;
use tokio_modbus::codec::rtu::ServerCodec;
use tokio_modbus::frame::rtu::ResponseAdu;
use tokio_modbus::frame::{ResponsePdu, SlaveRequest};
use tokio_modbus::{Request, Response};
use tokio_serial::SerialStream;
use tokio_util::codec::Framed;
#[derive(Parser, Debug)]
pub struct Slave {
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
    #[arg(default_value_t = 500, short)]
    timeout: u64,
    #[arg(default_value_t = 5000, short)]
    capacity: usize,
    #[arg(value_enum, default_value_t= LogLevel::Info, short)]
    pub log: LogLevel,
}

impl Slave {
    pub async fn action(&self) -> Result<()> {
        let buidler = serialport::new(self.path.clone(), self.baud_rate)
            .data_bits(self.data_bits.into())
            .parity(self.parity.into())
            .flow_control(self.flow_control.into())
            .stop_bits(self.stop_bits.into())
            .timeout(Duration::from_millis(self.timeout));
        _collect_data_origin_by_arg(buidler, self.capacity).await
    }
}

async fn _collect_data_origin_by_arg(builder: SerialPortBuilder, capacity: usize) -> Result<()> {
    let stream = SerialStream::open(&builder)?;
    let (coils, discrete, input_registers, hold_registers) = init(capacity);
    info!("打开串口成功");
    let mut framed = Framed::new(stream, ServerCodec::default());
    loop {
        let request = framed.next().await;
        match request {
            Some(Ok(req)) => {
                info!("recv req: {:?}", req);
                let hdr = req.hdr.clone();
                let req: SlaveRequest = req.into();
                let rsp = match &req.request {
                    Request::ReadInputRegisters(addr, len) => {
                        let first = *addr as usize;
                        let last = (addr + len - 1) as usize;
                        if first <= capacity && last <= capacity && *len > 0 {
                            let mut rs_data = Vec::with_capacity(*len as usize);
                            for register in input_registers[first..=last].iter() {
                                rs_data.push(*register);
                            }
                            Response::ReadInputRegisters(rs_data)
                        } else {
                            warn!(
                                "*addr <= max_len && (addr + len) < max_len: {}, {}",
                                addr, len
                            );
                            continue;
                        }
                    }
                    Request::ReadHoldingRegisters(addr, len) => {
                        let first = *addr as usize;
                        let last = (addr + len - 1) as usize;
                        if first <= capacity && last <= capacity && *len > 0 {
                            let mut rs_data = Vec::with_capacity(*len as usize);
                            for register in hold_registers[first..=last].iter() {
                                rs_data.push(*register);
                            }
                            Response::ReadHoldingRegisters(rs_data)
                        } else {
                            warn!(
                                "*addr <= max_len && (addr + len) < max_len: {}, {}",
                                addr, len
                            );
                            continue;
                        }
                    }
                    Request::ReadCoils(addr, len) => {
                        let first = *addr as usize;
                        let last = (addr + len - 1) as usize;
                        if first <= capacity && last <= capacity && *len > 0 {
                            let mut rs_data = Vec::with_capacity(*len as usize);
                            for register in coils[first..=last].iter() {
                                rs_data.push(*register);
                            }
                            Response::ReadCoils(rs_data)
                        } else {
                            warn!(
                                "*addr <= max_len && (addr + len) < max_len: {}, {}",
                                addr, len
                            );
                            continue;
                        }
                    }
                    Request::ReadDiscreteInputs(addr, len) => {
                        let first = *addr as usize;
                        let last = (addr + len - 1) as usize;
                        if first <= capacity && last <= capacity && *len > 0 {
                            let mut rs_data = Vec::with_capacity(*len as usize);
                            for register in discrete[first..=last].iter() {
                                rs_data.push(*register);
                            }
                            Response::ReadDiscreteInputs(rs_data)
                        } else {
                            warn!(
                                "*addr <= max_len && (addr + len) < max_len: {}, {}",
                                addr, len
                            );
                            continue;
                        }
                    }
                    _ => {
                        warn!("not not support ");
                        continue;
                    }
                };
                debug!("response");
                framed
                    .send(ResponseAdu {
                        hdr,
                        pdu: ResponsePdu(Ok(rsp)),
                    })
                    .await?;
                // let response_pdu = ResponsePdu(ResponsePdu);
            }
            Some(Err(e)) => {
                error!("{:?}", e);
            }
            None => {
                info!("recv none");
                break;
            }
        }
    }
    Ok(())
}

fn init(capacity: usize) -> (Vec<bool>, Vec<bool>, Vec<u16>, Vec<u16>) {
    let mut input_registers = Vec::with_capacity(capacity);
    let mut hold_registers = Vec::with_capacity(capacity);
    let mut coils = Vec::with_capacity(capacity);
    let mut discrete = Vec::with_capacity(capacity);

    for index in 0..capacity {
        input_registers.push((index % 10) as u16);
        hold_registers.push((index % 10) as u16);
        coils.push(index % 2 == 0);
        discrete.push(index % 2 == 0);
    }
    (coils, discrete, input_registers, hold_registers)
}
