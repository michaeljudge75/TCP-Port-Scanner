use std::time::Instant;
use tcp_port_scanner::results::{PortStatus, ScanError};
use tcp_port_scanner::scan_engine::*;
use tcp_port_scanner::scanner::rate_limit::*;

//Tests that scan_range() produces one entry per port in the range
#[test]
fn test_scan_range_length_is_correct() {
    let start = 10;
    let end = 20;

    let results = scan_range("127.0.0.1", start, end, 50, Some(10));

    assert_eq!(results.len(), (end - start + 1) as usize);

    for (i, (p, _)) in results.iter().enumerate() {
        assert_eq!(*p, start + i as u32);
    }
}

//Tests that scan_range keeps results in the same order as the ports
#[test]
fn test_scan_range_perserves_order() {
    let results = scan_range("127.0.0.1", 5, 8, 50, Some(10));

    let expected_ports = vec![5, 6, 7, 8];
    let returned_ports: Vec<u32> = results.into_iter().map(|(p, _)| p).collect();

    assert_eq!(expected_ports, returned_ports);
}

//Tests that rate limiter slows execution down
#[test]
fn test_scan_range_rate_limit_delay() {
    let start_port = 30000u32;
    let end_port = start_port + 2;

    let rate_limit = 1;

    let t0 = Instant::now();
    let _ = scan_range("127.0.0.1", start_port, end_port, 50, Some(rate_limit));
    let elapsed = t0.elapsed();

    assert!(elapsed.as_secs_f32() >= 1.5);
}

//Tests that scan_range propagates ScanError from scan_port
#[test]
fn test_scan_range_propgates_errors() {
    let results = scan_range("invalid.invalid.invalid", 1, 3, 50, Some(10));

    assert_eq!(results.len(), 3);

    for (_, result) in &results {
        assert!(matches!(result, Err(ScanError::DnsFailed(_))));
    }
}
