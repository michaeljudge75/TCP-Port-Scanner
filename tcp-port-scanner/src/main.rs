#![allow(unused)]
use tcp_port_scanner::cli::*;
use clap::Parser;
fn main() {
    parse_args(CliArgs::parse()).unwrap();

}
