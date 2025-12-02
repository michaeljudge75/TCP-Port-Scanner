use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::time::Duration;

use crate::results::{PortStatus, ScanError};

pub fn scan_port(host: &str, port: u32, timeout_ms: u64) -> Result<PortStatus, ScanError>{
    let addr = format!("{}:{}", host, port);
    
    let mut addrs_iter = match addr.to_socket_addrs(){
        Ok(iter) => iter,
        Err(_) => return Err(ScanError::DnsFailed(host.into())),
    };

    let socket_addr = match addrs_iter.next(){
        Some(a) => a,
        None => return Err(ScanError::UnknownHost),
    };

    match try_connect(socket_addr, timeout_ms){
        Ok(_) => Ok(PortStatus::Open),
        Err(_) => Ok(PortStatus::Closed),
    }
}

fn try_connect(addr: SocketAddr, timeout_ms: u64) -> std::io::Result<TcpStream>{
    TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms))
}
