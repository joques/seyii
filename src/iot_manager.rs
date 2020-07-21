/* 
iot_manager.rs
*/

mod iot;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::collections::HashMap;


#[derive(Debug)]
pub struct IoTManager<'a> {
 	active_iots_idents: Vec[&'a str],
 	failed_iots_idents: Vec[&'a str],
 	suspected_iots_idents: Vec[&'a str],
 	neighbourhood: 
 	head_iots: Vec<&'a iot::IOT<'a>>
 	iot_count: u32,
 }

impl<'a> IoTManager<'a> {
	pub fn create() -> Self {
		Self{
			active_iots: Vec::new(),
			failed_iots: Vec::new(),
			suspected_iots: Vec::new(),
			iot_count: 0,
			head_iots: Vec::new()
		}
	}

	fn create_iot() -> &'a iot::IOT {

	}

	pub fn handle_join() -> &'a iot::IOT {

	}
}