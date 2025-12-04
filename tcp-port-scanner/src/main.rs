#![allow(unused)]
use clap::Parser;
use tcp_port_scanner::cli::*;
use tcp_port_scanner::output::*;
use tcp_port_scanner::scan_engine::scan_range;
fn main() {
    let args = parse_args(CliArgs::parse()).unwrap();

    print_scan_summary(&args.target, args.port_start, args.port_end);

    //Actually Performs Scan
    let results = scan_range(
        &args.target,
        args.port_start,
        args.port_end,
        args.timeout_ms,
        args.rate_limit,
    );

    for (port, status) in results {
        match status {
            Ok(s) => println!("Port {}: {:?}", port, s),
            Err(e) => println!("Port {}: {:?}", port, e),
        }
    }
    //For Detailed Report
    /*
        for (port, result) in results{
            println!("{port}: {:?}", result);
        }
    */
}
