// cargo run --bin ch8-misc-wraperror
use std::{error, io};
use std::fmt;
use std::net;
use std::fs::File;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError { }

// We need to map_err here because we did not impl From, see wraperror2.rs
fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt")
        .map_err(UpstreamError::IO)?;

    let _localhost = "::1"
        .parse::<Ipv6Addr>()
        .map_err(UpstreamError::Parsing)?;

    println!("hello world from ch8-misc-wraperror");
    Ok(())
}