mod config;

use std::{fs, io};
use std::fmt::format;
use std::io::Error;
use std::process::Command;
use crate::config::Config;

fn install_dnsmasq() -> Result<(),Error>{
    let update_status = Command::new("sudo")
        .arg("apt-get")
        .arg("update")
        .status()?;

    if !update_status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to update package lists."));
    }

    // Install dnsmasq
    let install_status = Command::new("sudo")
        .arg("apt-get")
        .arg("install")
        .arg("-y")
        .arg("dnsmasq")
        .status()?;

    if !install_status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to install dnsmasq."));
    }

    println!("dnsmasq installed successfully.");
    Ok(())
}
fn write_dnsmasq_conf(conf: &str,path: &str) -> std::io::Result<()> {
    fs::write(path, conf)
}
fn generate_dnsmasq_conf(config: &Config) -> String {
    let mut conf = String::new();
    conf.push_str(&format!("port={}\n",&config.dhcp_config.port));
    for interface in &config.interfaces {
        conf.push_str(&format!("interface={}\n", interface.name));
        if interface.bind_interfaces {
            conf.push_str("bind-interfaces\n");
        }
        conf.push_str(&format!("dhcp-range={}\n", interface.dhcp_range));
        for lease in &interface.static_leases {
            conf.push_str(&format!(
                "dhcp-host={},{}\n",
                lease.mac, lease.ip
            ));
        }
        conf.push('\n'); // Add a newline for separation between interfaces
    }
    conf
}
fn main() {
    // match install_dnsmasq() {
    //     Ok(_) => println!("Setup completed."),
    //     Err(e) => eprintln!("Error during setup: {}", e),
    // }

    let config_content = fs::read_to_string("/home/betelgeuseee/projects/dhcp-server/Config.toml").unwrap();
    let config: Config= toml::from_str(&config_content).unwrap();
    let dnsmasq_conf = generate_dnsmasq_conf(&config);
    write_dnsmasq_conf(&dnsmasq_conf,&config.dhcp_config.conf).unwrap();
    println!("Successfully overridden the conf file");
}
