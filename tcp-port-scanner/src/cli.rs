//This File Contains Everything Relating to Parsing, Validating, and Documenting CLI including how
//help and usage text appears
use clap::Parser;

//Different Options for Command Line Arguments
#[derive(Parser, Debug, Clone)]
#[command(name = "TCP Port Scanner", version, author, about = "A Conccurrent TCP port scanner written in Rust")]
pub struct CliArgs{
    #[arg(help = "Target host to scan (e.g. 127.0.0.1 or example.com")]
    pub target: String,

    #[arg(short, long, default_value = "1-1024", help = "Port range to scan, e.g. 1-1024 or 80, 443")]
    pub ports: PortRange,

    #[arg(long, default_value = "connect", value_enum, help ="Scan mode: connect or timed")]
    pub mode: ScanMode,

    #[arg(long, default_value_t = 500, help = "Connection timeout in milliseconds")]
    pub timeout_ms: u64,

    #[arg(long, default_value_t = 100, help = "Max concurrent scans")]
    pub concurrency: usize,

    #[arg(long, help = "Limit scans per second (optional)")] 
    pub rate_limit: Option<u64>,

    #[arg(long, help = "Output file path for results (optional")]
    pub output_path: Option<String>,

    #[arg(short, long, help = "Enable verbose logging")]
    pub verbose: bool,
}
//Defines a start and end for port range
#[derive(Debug, Clone)]
pub struct PortRange{
    pub start: u16,
    pub end: u16,
}

//Represets how Scanner performs its scans
#[derive(Debug, Clone)]
pub enum ScanMode{
    Connect,
    Timed,
}

//Custom Error Type
#[derive(Debug, Clone)]
pub enum CliError{
    InvalidPortRange,
    InvalidTarget,
    IOError,
    InvalidArgument(String),
}

//Implemntations for Better Text Handling
impl std::fmt::Display for CliError{}
impl std::error::Error for CliError{}


//Helps Parse the CLI Arguments, wraps CliArgs::parse() and adds Validation
pub fn parse_args() -> Result<CliArgs, CliError>{
    let mut args = CliArgs::parse();

    if args.concurrency == 0{
        return Err(CliError::InvalidArgument("Concurrency must be greater than 0".into()));
    }

    if args.target.trim().is_empty(){
        return Err(CliError::InvalidTarget);
    }

    Ok(args)

}

//Helps Parse the Specified Ports into Numbers
use std::num::ParseIntError;
pub fn parse_port_range(input: &str) -> Result<PortRange, CliError>{
    let parts: Vec<&str> = input.split('-').collect();

    match parts.len(){
        1 =>{
            //Single Port
            let port: u16 = parts[0].parse().map_err(|_| CliError::InvalidPortRange)?;
            if port == 0 || port > 65535{
                return Err(CliError::InvalidPortRange);
            }
            Ok(PortRange{start: port, end: port})
        }
        2 =>{
            //Port Range    
            let start: u16 = parts[0].parse().map_err(|_| CliError::InvalidPortRange)?;
            let end: u16 = parts[1].parse().map_err(|_| CliError::InvalidPortRange)?;
            if start == 0 || end > 65535 || start > end{
                return Err(CliError::InvalidPortRange);
            }
            Ok(PortRange{start, end})
        }
        _ => Err(CliError::InvalidPortRange),
    }

}


//Tests For CLI Parsing
#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_args(){
        let args = CliArgs::parse_from(["prog", "127.0.0.1"]);
        assert_eq!(args.target, "127.0.0.1");
    }

}

#[cfg(test)]
mod tests{
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
