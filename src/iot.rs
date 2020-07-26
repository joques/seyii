/*
iot.rs
*/

use std::net::Ipv4Addr;

#[derive(Debug)]
struct IOT<'a> {
	iot_ident: String,
	iot_ip_address: Ipv4Addr,
 	iot_port: u32,
 	neighbours: Vec<&'a IOT<'a>>
}

impl<'a> IOT<'a> {
	pub fn create(device_count: u32, ip_addr: Ipv4Addr, port_number: u32) -> Self {
		let device_ident = format!("iot-{}", device_count);
		Self{
			iot_ident: device_ident,
			iot_ip_address: ip_addr,
			iot_port: port_number,
			neighbours: Vec::new()
		}
	}

	pub fn get_identifier(&self) -> String {
		self.iot_ident.clone()
	}

	pub fn add_neighbour(&mut self, niot: &'a IOT) -> Result<&str, &str> {
		self.neighbours.push(niot);
		Ok("IoT neighbour addition completed...")
	}
}