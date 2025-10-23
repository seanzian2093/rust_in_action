
use std::collections::BTreeMap;
use std::fmt;
use std::net::IpAddr;
use std::os::unix::io::AsRawFd;

use smoltcp::iface::{
    SocketSet,
    // EthernetInterfaceBuilder,
    // NeighborCache,
    Routes,
};
use smoltcp::phy::{
    wait as phy_wait,
    TunTapInterface,
};
use smoltcp::socket::{
    // SocketSet,
    tcp::Socket,
    tcp::SocketBuffer
};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};
use url::Url;
use crate::ethernet::MacAddress;

#[derive(Debug)]
enum HttpState {
    Connect,
    Request,
    Response,
}

#[derive(Debug)]
pub enum UpstreamError {
    Network(smoltcp::wire::Error),
    InvalidUrl,
    Content(std::str::Utf8Error),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<smoltcp::wire::Error> for UpstreamError {
    fn from(error: smoltcp::wire::Error) -> Self {
        UpstreamError::Network(error)
    }
}
impl From<std::str::Utf8Error> for UpstreamError {
    fn from(error: std::str::Utf8Error) -> Self {
        UpstreamError::Content(error)
    }
}

fn random_port() -> u16 {
    49152 + rand::random::<u16>() % 16384
}

pub fn get(
    tap: TunTapInterface,
    mac: MacAddress,
    add: IpAddr,
    url: Url,
) -> Result<(), UpstreamError> {
    Ok(())
}
