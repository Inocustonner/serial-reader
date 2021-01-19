use serialport;
use ansi_term::enable_ansi_support;
use std::env::args as argv;
use std::process::exit;
use std::io::prelude::*;
use std::time;

const ERR: &'static str = concat!("[ ", "\x1B[31m", "Err", "\x1B[0m", " ]");

fn show(buf: &[u8]) {
    for cc in buf {
        let c = *cc as char;
        if c.is_alphanumeric() {
            print!("{}", c);
        } else {
            print!("[{:X}]", c as u8);
        }
    }
    println!("");
}

fn main() {
    enable_ansi_support()
        .expect("Error occured during terminal asni color enabling");
    let args: Vec<String> = argv().collect();
    if args.len() < 2 {
        println!("Invalid number of arguments must be 1{}", ERR);
        exit(1);
    }
    let com = &args[1];
    println!("Openning {}", com);
    let mut serial = serialport::new(r"\\.\".to_owned() + com, 9600)
        .timeout(time::Duration::from_millis(500))
        .open()
        .expect("Couldn't open serial port");
    
    let mut buf = [0u8; 1024];
    loop {
        match serial.read(&mut buf) {
            Ok(n) => if n > 0 {show(&buf[..n])},
            Err(e) => if e.kind() != std::io::ErrorKind::TimedOut
                         {println!("Error occured {} {}", e, ERR)}
        }
    }
}
