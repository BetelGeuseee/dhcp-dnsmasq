use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub dhcp_config: DHCPConfig,
    pub interfaces: Vec<Interface>,
}
#[derive(Debug, Deserialize)]
pub struct DHCPConfig {
    pub conf: String,
    pub port: usize,
}
#[derive(Debug, Deserialize)]
pub struct Interface {
    pub name: String,
    pub bind_interfaces: bool,
    pub dhcp_range: String,
    pub static_leases: Vec<StaticLease>,
}

#[derive(Debug, Deserialize)]
pub struct StaticLease {
    pub mac: String,
    pub ip: String,
}