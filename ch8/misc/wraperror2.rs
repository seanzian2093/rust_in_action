// cargo run --bin ch8-misc-wraperror2
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

// We need to get rid of map_err so we neet to impl From
impl From<io::Error> for UpstreamError {
    fn from(err: io::Error) -> Self {
        UpstreamError::IO(err)
    }
}
impl From<net::AddrParseError> for UpstreamError {
    fn from(err: net::AddrParseError) -> Self {
        UpstreamError::Parsing(err)
    }
}
fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt")?;

    let _localhost = "::1".parse::<Ipv6Addr>()?;

    println!("hello world from ch8-misc-wraperror2");
    Ok(())
}
