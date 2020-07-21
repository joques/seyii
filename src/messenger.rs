/* 
messenger.rs 
*/

// Simple message for actual communication between clients
#[derive(Debug)]
pub struct SimpleMessage<'a> {
	pub mid: u32,
	pub sender: &'a str,
	pub topic: &'a str,
	pub content: &'a str,
}

impl<'a> SimpleMessage<'a> {
	pub fn create(message_count: u32, sender: &'a str, topic: &'a str, content: &'a str) -> Self {
		Self{
			mid: message_count,
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
