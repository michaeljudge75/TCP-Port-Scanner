#![allow(unused)]
use tcp_port_scanner::cli::*;
use tcp_port_scanner::output::*;
use clap::Parser;
fn main() {
    //parse_args(CliArgs::parse()).unwrap();
    let args = parse_args(CliArgs::parse()).unwrap();
    print_scan_summary(&args.target, args.port_start, args.port_end);
}
