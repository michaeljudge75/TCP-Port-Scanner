use crate::results::{PortStatus, ScanError};
use crate::scanner::connect::scan_port;
use crate::scanner::rate_limit::*;
use indicatif::ProgressBar;

pub fn scan_range(
    host: &str,
    start_port: u32,
    end_port: u32,
    timeout_ms: u64,
    rate_limit_per_sec: Option<u64>,
) -> Vec<(u32, Result<PortStatus, ScanError>)> {
    //Code for progress bar
    let total_ports = (end_port - start_port + 1) as u64;
    let pb = ProgressBar::new(total_ports);

    let mut limiter = rate_limit_per_sec.map(RateLimiter::new);
    let mut results = Vec::new();
    for port in start_port..=end_port {
        if let Some(l) = limiter.as_mut() {
            l.wait();
        }

        let result = scan_port(host, port, timeout_ms);

        results.push((port, result));

        pb.inc(1);
    }

    pb.finish_with_message("Scan Complete");

    results
}
