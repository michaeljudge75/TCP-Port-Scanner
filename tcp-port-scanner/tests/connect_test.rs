#![allow(unused)]
use tcp_port_scanner::scanner::connect::scan_port;
use tcp_port_scanner::results::{PortStatus, ScanError};

use std::net::TcpListener;

//Tests if Scanning an Open Port Works
#[test]
fn test_scan_open_port(){
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind test socket");
    let port = listener.local_addr().unwrap().port();

    let result = scan_port("127.0.0.1", port as u32, 200);

    assert!(matches!(result, Ok(PortStatus::Open)));
}

//Tests if Scanning a Closed Port Works
#[test]
fn test_scan_closed_port(){
   let result = scan_port("127.0.0.1", 65000, 200);

    assert!(matches!(result, Ok(PortStatus::Closed)));
}


