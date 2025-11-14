#![allow(unused)]
use tcp_port_scanner::output::*;
use tcp_port_scanner::results::*;

//Tests that Output Works Correctly
#[test]
fn test_scanresult_display(){
    let result = ScanResult{
        host: "127.0.0.1".into(),
        port: 80,
        status: PortStatus::Open,
    };

    assert_eq!(format!("{}", result), "127.0.0.1:80 - Open");
}
