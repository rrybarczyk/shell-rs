extern crate serial;
extern crate structopt;
extern crate xmodem;
#[macro_use]
extern crate structopt_derive;

use std::path::PathBuf;
use std::time::Duration;

use serial::core::{BaudRate, CharSize, FlowControl, SerialDevice, SerialPortSettings, StopBits};
use structopt::StructOpt;
use xmodem::{Progress, Xmodem};

mod parsers;

use parsers::{parse_baud_rate, parse_flow_control, parse_stop_bits, parse_width};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(
        short = "i",
        help = "Input file (defaults to stdin if not set)",
        parse(from_os_str)
    )]
    input: Option<PathBuf>,

    #[structopt(
        short = "b",
        long = "baud",
        parse(try_from_str = "parse_baud_rate"),
        help = "Set baud rate",
        default_value = "115200"
    )]
    baud_rate: BaudRate,

    #[structopt(
        short = "t",
        long = "timeout",
        parse(try_from_str),
        help = "Set timeout in seconds",
        default_value = "10"
    )]
    timeout: u64,

    #[structopt(
        short = "w",
        long = "width",
        parse(try_from_str = "parse_width"),
        help = "Set data character width in bits",
        default_value = "8"
    )]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(
        short = "f",
        long = "flow-control",
        parse(try_from_str = "parse_flow_control"),
        help = "Enable flow control ('hardware' or 'software')",
        default_value = "none"
    )]
    flow_control: FlowControl,

    #[structopt(
        short = "s",
        long = "stop-bits",
        parse(try_from_str = "parse_stop_bits"),
        help = "Set number of stop bits",
        default_value = "1"
    )]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn interact<T: SerialDevice>(port: &mut T, buffer: &[u8], raw: bool) -> std::io::Result<()> {
    if raw {
        port.write(&buffer[..])?;
    } else {
        Xmodem::transmit_with_progress(&buffer[..], port, progress_fn)?;
    }
    Ok(())
}

fn progress_fn(progress: Progress) {
    dbg!(progress);
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufReader, Read};

    let opt = Opt::from_args();

    // Define port
    let mut port = serial::open(&opt.tty_path).expect("path points to invalid TTY");
    port.set_timeout(Duration::from_secs(opt.timeout))
        .expect("could not set timeout");

    // Port Settings
    let mut settings = port.read_settings().unwrap();
    settings
        .set_baud_rate(opt.baud_rate)
        .expect("could not set baud rate");
    settings.set_char_size(opt.char_width);
    settings.set_stop_bits(opt.stop_bits);
    settings.set_flow_control(opt.flow_control);

    // Receive data to send
    let mut buffer = Vec::new();
    // bytes_read is a Result type containing the number of bytes read in
    let bytes_read: Result<usize, std::io::Error> = match opt.input {
        // Read data from file
        Some(f) => {
            let file = File::open(f).expect("bad file read");
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_end(&mut buffer)
        }
        None => {
            // Read data from stdin
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_end(&mut buffer)
        }
    };

    println!("wrote {} bytes to input", bytes_read.unwrap());
    interact(&mut port, &buffer, opt.raw).expect("could not interact with serial port");
}
