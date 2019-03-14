// The server program is
// https://github.com/taku-n/dbg or https://github.com/taku-n/srv

use std::fmt::Debug;
use std::io::prelude::*;
use std::net::TcpStream;

const ADDR_PORT: &str = "127.0.0.1:9841";

pub fn dbg<T>(t: T)
        where T: Debug {
    let stream = TcpStream::connect(ADDR_PORT);

    let mut stream = match stream {
        Ok(t) => t,
        Err(e) => {
            return
        },
    };

    let msg = format!("{:?}", t);  // Should this be a CString?
    stream.write(msg.as_bytes());
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dbg_test() {
        dbg("hello, world");
        dbg(2);
        dbg(3.0);
        dbg(&[2, 3, 5]);
        dbg(&[7.0, 11.0, 13.0]);
    }
}
