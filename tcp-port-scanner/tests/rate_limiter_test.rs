use std::time::{Duration, Instant};
use tcp_port_scanner::scanner::rate_limit::RateLimiter;

//Tests that new() stores the correct rate
#[test]
fn test_rate_limiter_new(){
    let limiter = RateLimiter::new(5);

    let expected = Duration::from_secs_f64(1.0/5.0);

    assert_eq!(limiter.interval(), expected);
}

//Tests that wait() must delay when called faster than the rate limit
#[test]
fn test_rate_limiter_enforces_delay(){
    let mut rl = RateLimiter::new(1);

    let start = Instant::now();
    rl.wait();
    rl.wait();

    let elapsed = start.elapsed();

    assert!(elapsed >= Duration::from_millis(900));
}

//Tests that high rate should not cause a notable delay
#[test]
fn test_rate_limiter_no_delay_high_rate(){
    let mut rl = RateLimiter::new(1000);

    let start = Instant::now();
    for _ in 0..20{
        rl.wait();
    }
    let elapsed = start.elapsed();

    assert!(elapsed < Duration::from_millis(50));
}

//Tests to ensure accurate timing for small sequences
#[test]
fn test_rate_limiter_timing_accuracy(){
    let mut rl = RateLimiter::new(2);

    let t0 = Instant::now();
    rl.wait();
    rl.wait();
    let elapsed = t0.elapsed();

    assert!(elapsed >= Duration::from_millis(450));
}

//Tests that multiple calls accumulate delay correctly 
#[test]
fn test_rate_limiter_multiple_waits(){
    let mut rl = RateLimiter::new(1);

    let t0 = Instant::now();
    rl.wait();
    rl.wait();
    rl.wait();
    let elapsed = t0.elapsed();

    assert!(elapsed >= Duration::from_secs(1) + Duration::from_millis(800));
}

//Tests to ensure wait() introduces a delay when required by the rate limit
#[test]
fn test_rate_limiter_waits(){
    let mut limiter = RateLimiter::new(1);
    limiter.wait();

    let start = Instant::now();
    limiter.wait();
    let elapsed = start.elapsed().as_secs_f64();

    assert!(elapsed >= 0.9, "Rate Limiter did not wait long enough");
}

//Tests that wait() introduces no delay if enough time has already passed
#[test]
fn test_rate_limiter_no_wait_after_interval(){
    let mut limiter = RateLimiter::new(10);
    limiter.wait();

    std::thread::sleep(Duration::from_millis(150));

    let start = Instant::now();
    limiter.wait();
    let elapsed = start.elapsed().as_secs_f64();

    assert!(elapsed < 0.05, "Rate Limiter waited even though interval elapsed");
}



