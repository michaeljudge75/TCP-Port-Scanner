use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;
#[derive(Debug, Clone, PartialEq)]
pub enum PortStatus{
    Open,
    Closed,
    Filtered,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScanResult{
    pub host: String,
    pub port: u32,
    pub status: PortStatus,
}

impl Display for ScanResult{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        write!(f, "{}:{} - {:?}", self.host, self.port, self.status)
    }
}
