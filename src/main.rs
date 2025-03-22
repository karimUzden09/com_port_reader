use std::{
    io::{Write, stdout},
    thread,
    time::Duration,
};

use clap::Parser;
use serial2::{CharSize, FlowControl, Parity, SerialPort, Settings, StopBits};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.
#[derive(Debug, Parser)]
pub struct Args {
    /// Выставления COM порта который вы хотите слушать
    #[arg(long)]
    serial: String,
    /// Выставления RAW COM порта
    #[arg(long)]
    raw: bool,
    #[arg(long)]
    /// Выставления Char size COM порта от 5 до 8
    char_size: u8,
    /// Выставления Stop bits COM порта значения 1 либо 2
    #[arg(long)]
    stop_bits: u8,
    /// Выставления Parity COM порта доступные значения None, Odd, Even
    #[arg(long)]
    set_parity: String,
    /// Выставления Flow control COM порта доступные значения None, RtsCts, XonXoff
    #[arg(long)]
    set_flow_control: String,
    /// Выставления скорсти передачи по порту
    #[arg(long, default_value_t = 11520)]
    rate: u32,
    /// Выставление sleep в секундах
    #[arg(long)]
    seconds: u64,
    /// Выставление DTR
    #[arg(long ,action = clap::ArgAction::SetFalse, default_value_t = true)]
    dtr: bool,
    /// Выставление RTS
    #[arg(long ,action = clap::ArgAction::SetFalse, default_value_t = true)]
    rts: bool,
}

fn parse_char_size(char_size: u8) -> CharSize {
    match char_size {
        5 => CharSize::Bits5,
        6 => CharSize::Bits6,
        7 => CharSize::Bits7,
        8 => CharSize::Bits8,
        _ => unimplemented!(),
    }
}

fn parse_stop_bits(stop_bits: u8) -> StopBits {
    match stop_bits {
        1 => StopBits::One,
        2 => StopBits::Two,
        _ => unimplemented!(),
    }
}
fn parse_paryty(value: &str) -> Parity {
    match value {
        "None" => Parity::None,
        "Odd" => Parity::Odd,
        "Even" => Parity::Even,
        _ => unimplemented!(),
    }
}
fn parse_flow_controll(value: &str) -> FlowControl {
    match value {
        "None" => FlowControl::None,
        "RtsCts" => FlowControl::RtsCts,
        "XonXoff" => FlowControl::XonXoff,
        _ => unimplemented!(),
    }
}
fn get_serial(args: &Args) -> Result<SerialPort> {
    let serial = SerialPort::open(args.serial.clone(), |mut settings: Settings| {
        if args.raw {
            settings.set_raw();
        }
        settings.set_baud_rate(args.rate)?;
        settings.set_char_size(parse_char_size(args.char_size));
        settings.set_stop_bits(parse_stop_bits(args.stop_bits));
        settings.set_parity(parse_paryty(&args.set_parity));
        settings.set_flow_control(parse_flow_controll(&args.set_flow_control));
        Ok(settings)
    })?;
    Ok(serial)
}
fn main() -> Result<()> {
    let args = Args::parse();
    let port = get_serial(&args)?;
    port.set_dtr(args.dtr)?;
    port.set_rts(args.rts)?;

    let mut buffer = vec![];
    loop {
        match port.read(&mut buffer) {
            Ok(_readed) => {
                stdout().write_all(&buffer)?;
                stdout().flush()?;
            }
            Err(err) => {
                println!("Can not read from port: {:?}", err);
            }
        }
        thread::sleep(Duration::from_secs(args.seconds));
    }
}
