use tcp_port_scanner::cli::*;
use clap::Parser;
//Tests that the Default Arguments are working
#[test]
fn test_defaults(){
    let args = CliArgs::parse_from(["tcp-port-scanner", "--target", "127.0.0.1"]);
    assert_eq!(args.target, "127.0.0.1");
}

//Tests that Port Start Cannot be Greater than Port End
#[test]
fn test_start_greater(){
    let range = parse_args(CliArgs::parse_from(["tcp-port-scanner", "--port-start", "80", "--port-end", "20"])); 
    assert!(matches!(range, Err(CliError::InvalidPortRange)));
}
