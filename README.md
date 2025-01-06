# Rust DHCP Server

A simple DHCP server written in Rust that leverages `dnsmasq` for managing DHCP leases and network configurations. This project allows you to configure multiple network interfaces, set DHCP ranges, and assign static IP addresses based on MAC addresses.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Testing](#testing)
- [Project Structure](#project-structure)


## Features

- **Configurable Network Interfaces**: Manage multiple network interfaces with customizable DHCP ranges.
- **Static DHCP Leases**: Assign fixed IP addresses to devices based on their MAC addresses.
- **Automated `dnsmasq` Setup**: Automatically installs and configures `dnsmasq` based on your configuration.
- **Flexible Configuration**: Easy-to-edit `Config.toml` for setting up your DHCP server parameters.

## Prerequisites

- **Operating System**: Debian-based Linux distribution (e.g., Ubuntu, Raspberry Pi OS)
- **Rust**: Installed with `cargo` ([Installation Guide](https://www.rust-lang.org/tools/install))
- **Root Privileges**: Required for installing packages and configuring network interfaces

## Installation

1. **Clone the Repository**

    ```bash
    git clone https://github.com/BetelGeuseee/dhcp-dnsmasq.git
    ```

2. **Install Dependencies**

   The project relies on `toml` and `serde` crates, which are specified in the `Cargo.toml` and will be automatically installed when you build the project.

3. **Build the Project**

    ```bash
    cargo build --release
    ```

   The compiled binary will be located at `./target/release/dhcp-server`.

## Configuration

The DHCP server is configured using the `Config.toml` file. Below is an example configuration:

### `Config.toml`

```toml
[dhcp_config]
conf = "/etc/dnsmasq.conf"
port = 0 # disable dns in dnsmasq

# ==========================
# dnsmasq Configuration File
# ==========================

# Define the network interfaces to be managed by dnsmasq
[[interfaces]]
# Name of the network interface (e.g., eth0)
name = "raspberry"

# Whether to bind dnsmasq explicitly to the interface
bind_interfaces = true

# DHCP range in the format: start_ip, end_ip, lease_time
dhcp_range = "192.168.50.50,192.168.50.150,12h"

# List of static DHCP leases based on MAC addresses
[[interfaces.static_leases]]
# MAC address of the device
mac = "92:01:c8:a9:b2:c0"

# Reserved IP address for the device
ip = "192.168.50.100"

# Uncomment and configure additional static leases as needed
# [[interfaces.static_leases]]
# mac = "11:22:33:44:55:66"
# ip = "192.168.50.101"

# ==========================
# Additional Interfaces (Optional)
# ==========================
# To manage multiple interfaces, replicate the [[interfaces]] table.

# [[interfaces]]
# name = "eth1"
# bind_interfaces = true
# dhcp_range = "192.168.51.10,192.168.51.100,12h"

# [[interfaces.static_leases]]
# mac = "CC:DD:EE:FF:00:11"
# ip = "192.168.51.50"
```

**Configuration Options:**

- **[dhcp_config]**
    - `conf`: Path to the `dnsmasq` configuration file.
    - `port`: Port number for `dnsmasq` DNS service (`0` to disable DNS).

- **[[interfaces]]**
    - `name`: Name of the network interface (e.g., `eth0`, `raspberry`).
    - `bind_interfaces`: Boolean to explicitly bind `dnsmasq` to the interface.
    - `dhcp_range`: DHCP IP range in the format `start_ip, end_ip, lease_time`.

    - **[[interfaces.static_leases]]**
        - `mac`: MAC address of the device.
        - `ip`: Reserved IP address for the device.

## Usage

1. **Configure the DHCP Server**

   Edit the `Config.toml` file to match your network setup and desired DHCP configurations.

2. **Run the DHCP Server**

    ```bash
    sudo ./target/release/dhcp-server
    ```

   This will:

    - Install `dnsmasq` if it's not already installed.
    - Generate the `dnsmasq` configuration file based on `Config.toml`.
    - Override the existing `dnsmasq` configuration with the generated one.

3. **Restart `dnsmasq`**

   After running the DHCP server, restart `dnsmasq` to apply the new configurations:

    ```bash
    sudo systemctl restart dnsmasq
    ```

## Testing

To test the DHCP server setup, follow these steps to create virtual network interfaces and simulate DHCP requests.

1. **Create a Virtual Ethernet (veth) Pair**

   Name the ends of the veth pair as `raspberry` (to represent the Raspberry Pi interface) and `mindray` (to represent the Mindray monitor interface):

    ```bash
    sudo ip link add raspberry type veth peer name mindray
    ```

2. **Verify the Interfaces**

    ```bash
    ip link show
    ```

   You should see both `raspberry` and `mindray` interfaces listed.

3. **Assign ip addresses to both interfaces
   ```bash
   sudo ip addr add 192.168.50.1/24 dev mindray
   sudo ip addr add 192.169.50.2/24 dev raspberry
   ```

4. **Use the MAC Address from the `mindray` Interface and Map it to IP in Rust Config**

    - Retrieve the MAC address of the `mindray` interface:

        ```bash
        ip link show mindray
        ```

      Example output:

        ```
        5: mindray: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP mode DEFAULT group default qlen 1000
            link/ether aa:bb:cc:dd:ee:ff brd ff:ff:ff:ff:ff:ff
        ```

      Note the MAC address (`aa:bb:cc:dd:ee:ff` in this example).

    - Update your `Config.toml` to map this MAC address to a desired IP:

        ```toml
        [[interfaces.static_leases]]
        mac = "aa:bb:cc:dd:ee:ff"
        ip = "192.168.50.101"
        ```

6. **Bring Up the Interfaces**

    ```bash
    sudo ip link set mindray up
    sudo ip link set raspberry up
    ```

7. **Install DHCP Client**

    ```bash
    sudo apt-get install isc-dhcp-client
    ```

8. **Send DHCP Request from `mindray` Interface**

    ```bash
    sudo dhclient mindray
    ```

   This will send a DHCP request from the `mindray` interface and should receive an IP address based on your `Config.toml` settings.

9. **Verify the IP Address Assignment**

    ```bash
    ip addr show mindray
    ```

   Ensure that `mindray` has been assigned the correct IP address as per your static lease configuration (e.g., `192.168.50.101`).

10. **Clean Up the veth Pair**

   After testing, delete the `mindray` interface. Both ends of the veth pair (`raspberry` and `mindray`) will be removed:

    ```bash
    sudo ip link delete mindray
    ```

## Project Structure

```
dhcp-server/
├── Cargo.toml
├── Config.toml
├── README.md
└── src
    ├── config.rs
    └── main.rs
```

- **Cargo.toml**: Rust project configuration, including dependencies.
- **Config.toml**: Configuration file for the DHCP server.
- **src/main.rs**: Main application logic for setting up and configuring `dnsmasq`.
- **src/config.rs**: Configuration structures for parsing `Config.toml`.

---

