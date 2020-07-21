/*
iot.rs
*/

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
struct IOT<'a> {
	iot_ident: &'a str,
	iot_ip_address: IpAddr,
 	iot_port: u32,
 	neighbours: Vec<&'a IOT<'a>>
}

impl<'a> IOT<'a> {
	pub fn create(ident: &'a str, ip_addr: IpAddr, port_number: u32) -> Self {
		Self{
			iot_ident: ident,
			iot_ip_address: ip_addr,
			iot_port: port_number,
			neighbours: Vec::new()
		}
	}

	pub fn add_neighbour(&mut self, niot: &'a IOT) -> Result<&str, &str> {
		self.neighbours.push(niot);
		Ok("IoT neighbour addition completed...")
	}
}