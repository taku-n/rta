// The server program is https://github.com/taku-n/srv

use std::io::prelude::*;
use std::net::TcpStream;

pub fn str(s: &str) {
    let stream = TcpStream::connect("127.0.0.1:8800");

    let mut stream = match stream {
        Ok(t) => t,
        Err(e) => {
            return
        },
    };

    stream.write(s.as_bytes());
}

pub fn i32(x: i32) {
    str(&x.to_string());
}

pub fn f64(x: f64) {
    str(&x.to_string());
}

pub fn s_i32(s: &[i32]) {
    str(&format!("{:?}", s));
}

pub fn s_f64(s: &[f64]) {
    str(&format!("{:.6?}", s));
}
