/*
    Copyright (C) 2024 FloppyDisk
    https://github.com/flpydsk/subcalc.git

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; version 2 only.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use ipnetwork::IpNetwork;
use serde::{Serialize, Deserialize};
use serde_json;
use std::env;

#[derive(Clone, Serialize, Deserialize)]
struct SubnetInfo {
    ip_version: u8,
    subnet: String,
    prefix_length: u8,
    num_ips: Option<u128>,
    range_start: String,
    range_end: String,
}

impl Default for SubnetInfo {
    fn default() -> Self {
        SubnetInfo {
            ip_version: 0,
            subnet: String::new(),
            prefix_length: 0,
            num_ips: Some(0),
            range_start: String::new(),
            range_end: String::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct JsonResponse {
    result: SubnetInfo,
    success: bool,
    error: String,
}

fn get_subnet(subnet: &str) -> JsonResponse {
    if let Ok(ip_network) = subnet.parse::<IpNetwork>() {
        let num_ips: Option<u128> = match ip_network.ip() {
            std::net::IpAddr::V4(_) => 2u128.checked_pow(32 - u32::from(ip_network.prefix())),
            std::net::IpAddr::V6(_) => {
                let prefix_length: u32 = u32::from(ip_network.prefix());
                if prefix_length == 0 {
                    Some(u128::MAX)
                } else {
                    2u128.checked_pow(128 - u32::from(prefix_length))
                }
            },
        };
        JsonResponse {
            result: SubnetInfo {
                ip_version: match ip_network.ip() {
                    std::net::IpAddr::V4(_) => 4,
                    std::net::IpAddr::V6(_) => 6,
                },
                subnet: subnet.to_string(),
                prefix_length: ip_network.prefix(),
                num_ips,
                range_start: ip_network.network().to_string(),
                range_end: ip_network.broadcast().to_string(),
            },
            success: true,
            error: String::new(),
        }
    } else {
        JsonResponse {
            result: SubnetInfo::default(),
            success: false,
            error: "Invalid subnet format".to_string(),
        }
    }
}

fn return_data(response: &JsonResponse, exit_code: i32) {
    let json_str: String = serde_json::to_string_pretty(&response).unwrap();
    println!("{}", json_str);
    std::process::exit(exit_code);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let response: JsonResponse = JsonResponse {
            result: SubnetInfo::default(),
            success: false,
            error: "No Subnet Specified".to_string(),
        };
        return_data(&response, 1);
    }
    let subnet: &String = &args[1];
    return_data(&get_subnet(subnet), 0);
}
