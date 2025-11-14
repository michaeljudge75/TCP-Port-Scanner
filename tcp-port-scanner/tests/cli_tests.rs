use tcp_port_scanner::cli::*;
use clap::Parser;
//Tests that the Default Arguments are working
#[test]
fn test_defaults(){
    let args = CliArgs::parse_from(["tcp-port-scanner", "--target", "127.0.0.1"]);
    assert_eq!(args.target, "127.0.0.1");
}

//Tests that Port Range is working
#[test]
fn test_port_range(){
    let args = CliArgs::parse_from(["tcp-port-scanner", "--port-start", "22", "--port-end", "88"]);
    
    let parsed = parse_args(args).unwrap();
    assert_eq!(parsed.port_start, 22);
    assert_eq!(parsed.port_end, 88);
}

//Tests that Single Port is Working
#[test]
fn test_single_port(){
    let args = CliArgs::parse_from(["tcp-port-scanner", "--port-single", "20"]);
    let parsed = parse_args(args).unwrap();
    assert_eq!(parsed.port_single, 20);
    assert_eq!(parsed.port_start, 20);
    assert_eq!(parsed.port_end, 21);
}
//Tests that Port Start Cannot be Greater than Port End
#[test]
fn test_start_greater(){
    let range = parse_args(CliArgs::parse_from(["tcp-port-scanner", "--port-start", "80", "--port-end", "20"])); 
    assert!(matches!(range, Err(CliError::InvalidPortRange)));
}

//Tests that Port Cannot Have Invalid Ports
#[test]
fn test_invalid_args(){
    let range = parse_args(CliArgs::parse_from(["tcp-port-scanner", "--port-start", "80000", "--port-end", "90000"]));
    assert!(matches!(range, Err(CliError::InvalidPortRange)));
}

//Tests that IP is Valid
#[test]
fn test_valid_ip_parses(){
    let args = CliArgs::parse_from(["tcp-port-scanner", "--target", "127.0.0.1", "--port-start", "20", "--port-end", "80"]);
    let parsed = parse_args(args);
    assert!(parsed.is_ok());
}

//Tests that Code Errors if IP is Invalid
/*
#[test]
fn test_invalid_ip_fails(){
    let args = parse_args(CliArgs::parse_from(["tcp-port-scanner", "--target", "999.999.999.999", "--port-start", "20", "--port-end", "80"]));
    assert!(matches!(args, Err(CliError::InvalidTarget)));
}
*/
//Tests that Domain is Valid
#[test]
fn test_valid_domain_parses(){
    let args = CliArgs::parse_from(["tcp-port-scanner", "--target", "example.com", "--port-start", "20", "--port-end", "80"]);
    let parsed = parse_args(args);
    assert!(parsed.is_ok());
}

//Tests that Code Errors if Domain is Invalid
/*
#[test]
fn test_invalid_domain_fails(){
    let args = parse_args(CliArgs::parse_from(["tcp-port-scanner", "--target", "not_a_real_domain_please.fail", "--port-start", "20", "--port-end", "80"]));
    assert!(matches!(args, Err(CliError::InvalidTarget)));
}
*/
