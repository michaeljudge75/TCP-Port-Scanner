use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
#[derive(Debug, Clone, PartialEq)]
pub enum PortStatus {
    Open,
    Closed,
    Filtered,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanError{
    DnsFailed(String),
    UnknownHost
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScanResult {
    pub host: String,
    pub port: u32,
    pub status: PortStatus,
}

impl Display for ScanResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}:{} - {:?}", self.host, self.port, self.status)
    }
}
