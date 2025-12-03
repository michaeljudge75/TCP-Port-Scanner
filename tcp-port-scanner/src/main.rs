#![allow(unused)]
use tcp_port_scanner::cli::*;
use tcp_port_scanner::output::*;
use tcp_port_scanner::scan_engine::scan_range;
use clap::Parser;
fn main() {
    let args = parse_args(CliArgs::parse()).unwrap();

    //Actually Performs Scan    
    let results = scan_range(
        &args.target,
        args.port_start,
        args.port_end,
        args.timeout_ms,
        args.rate_limit.unwrap(),
    );

    print_scan_summary(&args.target, args.port_start, args.port_end);

    //For Detailed Report   
    /*
    for (port, result) in results{
        println!("{port}: {:?}", result);
    }
*/
}
