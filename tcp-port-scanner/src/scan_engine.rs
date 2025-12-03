use crate::scanner::rate_limit::*;
use crate::scanner::connect::scan_port;
use crate::results::{PortStatus, ScanError};

pub fn scan_range(
    host: &str,
    start_port: u32,
    end_port: u32,
    timeout_ms: u64,
    rate_limit_per_sec: u64,
) -> Vec<(u32, Result<PortStatus, ScanError>)> {
    let mut limiter = RateLimiter::new(rate_limit_per_sec);
    let mut results = Vec::new();
    for port in start_port..=end_port{
        limiter.wait();

        let result = scan_port(host, port, timeout_ms);

        results.push((port, result));
    }

    results
}
