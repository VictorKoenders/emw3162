#![no_std]
#![no_main]

use mxchip_wnet::net::TcpStream;
use mxchip_wnet::{
    net::{Read, Write},
    CStr, MxChip,
};

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::fmt::Write as _;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

#[entry]
fn main() -> ! {
    let mut hstdio = hio::hstdout().unwrap();

    let chip = MxChip::init().unwrap();
    let host = CStr::from_bytes_with_nul(b"google.com\0").unwrap();
    let mut stream = TcpStream::connect(&chip, host, 80).unwrap();
    stream.write(b"GET / HTTP/1.1\r\n\r\n").unwrap();
    let mut buffer = [0u8; 1024];
    let len = stream.read(&mut buffer).unwrap();

    writeln!(hstdio, "{:?}", core::str::from_utf8(&buffer[..len])).unwrap();

    loop {
        asm::nop();
    }
}
