//This File Contains Everything Relating to Parsing, Validating, and Documenting CLI including how help and usage text appears
#![allow(unused)]
use clap::Parser;
use std::fmt::Formatter;

//Different Options for Command Line Arguments
#[derive(Parser, Debug, Clone, PartialEq)]
#[command(name = "TCP Port Scanner", version = "1.0", author = "Michael Judge", about = "A Concurrent TCP port scanner written in Rust")]
pub struct CliArgs{
    #[arg(long, default_value ="127.0.0.1", help = "Target host to scan (e.g. 127.0.0.1 or example.com)")]
    pub target: String,

    #[arg(long, default_value_t = 0, help = "Use if you only want to scan a single port")]
    pub port_single: u32,

    #[arg(long, default_value_t = 0, help = "What port the scan starts at")]
    pub port_start: u32,

    #[arg(long, default_value_t = 65535, help = "What port the scan stops at")]
    pub port_end: u32,

    #[arg(long, default_value = "connect", value_enum, help ="Scan mode: connect or timed")]
    pub mode: ScanMode,

    #[arg(long, default_value_t = 500, help = "Connection timeout in milliseconds")]
    pub timeout_ms: u64,

    #[arg(long, default_value_t = 100, help = "Max concurrent scans")]
    pub concurrency: usize,

    #[arg(long, help = "Limit scans per second (optional)")] 
    pub rate_limit: Option<u64>,

    #[arg(short, long, help = "Enable verbose logging")]
    pub verbose: bool,
}

//Represets how Scanner performs its scans
#[derive(Debug, Clone, clap::ValueEnum, PartialEq)]
pub enum ScanMode{
    Connect,
    Timed,
}

//Custom Error Type
#[derive(Debug, Clone, PartialEq)]
pub enum CliError{
    InvalidPortRange,
    InvalidTarget,
    IOError,
    InvalidArgument(String),
}

//Helps Parse the CLI Arguments, wraps CliArgs::parse() and adds Validation
use std::net::ToSocketAddrs;
pub fn parse_args(mut args: CliArgs) -> Result<CliArgs, CliError>{
    if args.port_single != 0{
        args.port_start = args.port_single;
        args.port_end = args.port_single+1;
    }

    if args.port_start == 0 || args.port_end > 65535 || args.port_start > args.port_end{
        return Err(CliError::InvalidPortRange);
    }

    if args.concurrency == 0{
        return Err(CliError::InvalidArgument(
            "Concurrency must be greater than 0".into()
        ));
    }

    let target = format!("{}:80", args.target);
    if target.to_socket_addrs().is_err(){
        return Err(CliError::InvalidArgument(
            "Address is Invalid".into(),
        ));
    }

    if let Some(rate_limit) = args.rate_limit{
        if rate_limit == 0{
            return Err(CliError::InvalidArgument(
                "Rate limit must be greater than 0".into(),
            ));
        }
    }
    print!("{:#?}", args);
    Ok(args)
}


//Tests For CLI Parsing
/*
#[cfg(test)]
mod tests{
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_args(){
        let args = CliArgs::parse_from(["prog", "127.0.0.1"]);
        assert_eq!(args.target, "127.0.0.1");
    }


    #[cfg(test)]

    use super::*;
    #[test]
    fn test_parse_single_port(){
        let range = parse_port_range("22").unwrap();
        assert_eq!(range.start, 22);
        assert_eq!(range.end, 22);
    }
    #[test]
    fn test_parse_port_range(){
        let range = parse_port_range("20-25").unwrap();
        assert_eq!(range.start, 20);
        assert_eq!(range.end, 25);
    }
    #[test]
    fn test_invaild_range(){
        assert!(parse_port_range("70000-80000").is_err());
    }
}
*/
