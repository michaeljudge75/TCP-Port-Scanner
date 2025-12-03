use std::time::{Duration, Instant};
use std::thread::sleep;

#[derive(Debug)]
pub struct RateLimiter{
    interval: Duration,
    last: Instant,
}

impl RateLimiter{
    pub fn new(request_per_second: u64) -> Self{
        let interval = Duration::from_secs_f64(1.0/request_per_second as f64);
        Self{
            interval,
            last: Instant::now(),
        }
    }

    pub fn wait(&mut self){
        let elapsed = self.last.elapsed();
        if elapsed < self.interval{
            sleep(self.interval - elapsed);
        }
        self.last = Instant::now();
    }
    
    pub fn interval(&self) -> Duration{
        self.interval
    }


}
