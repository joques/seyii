/* 
messenger.rs 
*/

// Simple message for actual communication between clients
#[derive(Debug)]
pub struct SimpleMessage<'a> {
	pub mid: String,
	pub sender: &'a str,
	pub topic: &'a str,
	pub content: &'a str,
}

impl<'a> SimpleMessage<'a> {
	pub fn create(message_count: u32, sender: &'a str, topic: &'a str, content: &'a str) -> Self {
		let simple_msg_full_ident = format!("s-msg-{}", message_count);
		Self{
			mid: simple_msg_full_ident,
			sender,
			topic,
			content,
		}
	}
}

impl PartialEq for SimpleMessage<'_> {
	fn eq(&self, other_message: &Self) -> bool {
		self.mid == other_message.mid
	}
}

impl PartialEq<SimpleMessage<'_>> for &'_ SimpleMessage<'_> {
    fn eq(&self, other: &SimpleMessage) -> bool {
        self.mid == other.mid
    }
}


#[derive(Debug)]
pub struct JoinMessage {
	host_address: (u8,u8,u8,u8),
	port_number: u32,
}

impl JoinMessage {
	pub fn create(host_address: (u8,u8,u8,u8), port_number: u32) -> Self {
		Self{
			host_address,
			port_number,
		}
	}
}

#[derive(Debug)]
pub enum MembershipCategory {
	Suspicion,
	Failure,
	Resurrect,
}

#[derive(Debug)]
pub struct MembershipMessage {
	category: MembershipCategory,
	iot_ident: String
}

impl MembershipMessage {
	pub fn create(cat: MembershipCategory, content: String) -> Self {
		Self {
			category: cat,
			iot_ident: content
		}
	}

	pub fn get_category(&self) -> &MembershipCategory {
		&self.category
	}

	pub fn get_content(&self) -> String {
		self.iot_ident.clone()
	}
}