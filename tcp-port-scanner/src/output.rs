#![allow(unused)]
use crate::results::{ScanResult, PortStatus};

//Prints a summary of what is being scanned
pub fn print_scan_summary(target: &str, start_port: u32, end_port: u32){
    println!("\nStarting scan on target: {}", target);
    println!("Port Range: {} - {}", start_port, end_port);
    println!("-----------------------------------------");
} 

//Prints what ports have been scanned and what their status is
pub fn print_port_status(result: &ScanResult){
    println!("Port {:>5}: {:?}", result.port, result.status);
}

//Prints full results of all ports scanned and a total of how many ports were scanned
pub fn print_results(results: &Vec<ScanResult>){
    println!("\nScan Complete!");
    println!("--------------------");

    for result in results{
        println!("{}", result); 
    }

    println!("--------------------");
    println!("Total Ports Scanned: {}", results.len());
}

//Prints a message if there is any error
pub fn print_error(msg: &str){
    eprintln!("[ERROR] {}", msg);
}
