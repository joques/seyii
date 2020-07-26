/* 
iot_manager.rs
*/

mod iot;
mod messenger;
use std::net::Ipv4Addr;
use std::collections::HashMap;


#[derive(Debug)]
pub struct IoTManager {
 	active_iots_idents: Vec[String],
 	failed_iots_idents: Vec[String],
 	suspected_iots_idents: Vec[String],
 	neighbourhood: HashMap<String, Vec<String>>,
 	head_iots: Vec<String>,
 	iot_count: u32,
 	created_iots: Vec<iot::IOT>,
 }

impl IoTManager {
	// create an instance of IotManager
	pub fn create() -> Self {
		Self{
			active_iots: Vec::new(),
			failed_iots: Vec::new(),
			suspected_iots: Vec::new(),
			iot_count: 0,
			head_iots: Vec::new(),
			neighbourhood: HashMap::new(),
			created_iots: Vec::new()
		}
	}

	// handle a request for an IOT to join the infrastrcuture
	pub fn handle_join_message(&mut self, msg: &'a messenger::JoinMessage) -> Result<&str, &str> {
		let ip_addr = Ipv4Addr::new(msg.host_address.0, msg.host_address.1, msg.host_address.2, msg.host_address.3);
		let new_iot = iot::IOT::create(self.iot_count, ip_addr, msg.port_number);
		self.iot_count +=1;
		self.active_iots.push(new_iot.get_identifier());
        self.created_iots.push(new_iot);
        Ok("New IOT Created...")
	}

	pub fn add_neighbour(&mut self, source_iot_ident: String, neighbour_iot_ident: String) -> Result<&str, &str> {
		match self.neighbourhood.get_mut(source_iot_ident) {
			// there is no neighbour for this iot yet
			None => {
				let mut neighbour_col = Vec::new();
				neighbour_col.push(neighbour_iot_ident);
				self.neighbourhood.insert(source_iot_ident, neighbour_col);
				Ok("First neighbour added for iot device")
			}
			Some(all_neighbs) => {
				if all_neighbs.iter().any(|cur_neigh| cur_neigh == neighbour_iot_ident) {
					Err("Error! Neighbour already added for iot device")
				} else {
					all_neighbs.push(neighbour_iot_ident);
					Ok("Added another neighbour for iot device")
				}
			}
		}
	}

	pub fn add_head_iot(&mut self, new_head_iot_ident: String) -> Result<&str, &str> {
		if self.head_iots.iter().any(|cur_head_ident| cur_head_ident == new_head_iot_ident) {
			Err("Error! This iot device has already been set as head")
		} else {
			self.head_iots.push(new_head_iot_ident);
			Ok("New iot device added as head")
		}
	}

	fn is_iot_suspected(&self, iot_ident: &String) -> bool {
		self.suspected_iots_idents.iter().any(|cur_iot_ident| *cur_iot_ident == *iot_ident)
	}

	fn has_iot_failed(&self, iot_ident: &String) -> bool {
		self.failed_iots_idents.iter().any(|cur_iot_ident| *cur_iot_ident == *iot_ident)
	}

	pub fn handle_membership_message(&mut self, msg: &'a messenger::MembershipMessage) -> Result<&str, &str> {
		let msg_content = msg.get_content();
		// here we shall handle iot failure and suspicion
		match msg.get_category() {
			// handle suspicion case
			messenger::MembershipCategory::Suspicion => {
				if self.suspected_iots_idents.iter().any(|cur_suspect_ident| cur_suspect_ident == msg_content) {
					Ok("Failure suspicion confirmned for IoT device")
				} else {
					self.suspected_iots.push(msg_content);
					Ok("New failure suspicion added for IoT device")
				}
			}
			// handle failure case
			messenger::MembershipCategory::Failure => {
				if !self.is_iot_suspected(&msg_content) {
					Err("Error! This device has never been suspected of failing before")
				} else {
					if !self.has_iot_failed(&msg_content) {
						Ok("Failure acknowledgment for iot device")
					} else {
						self.failed_iots_idents.push(msg_content);
						Ok("Failure confirmed for IoT device")
					}
				}
			}

			// resurrect from suspicion
			messenger::MembershipCategory::Resurrect => {
				if self.has_iot_failed(&msg_content) {
					Err("Error! the iot device has been declared failed")
				} else if self.is_iot_suspected(&msg_content) {
					// remove the ident from suspected
					self.suspected_iots.retain(|&cur_iot_ident| cur_iot_ident != msg_content);
					Ok("IoT device resurrected")
				} else {
					Ok("IoT device back to active list")
				}
			}
		}
	}

	pub fn handle_simple_message(&mut self, msg: &'a messenger::SimpleMessage<'a>) -> Result<&str, &str> {
		// code goes here...
		// check the ize requirements before sending the message

		// add a copty to the buffer
	}
}