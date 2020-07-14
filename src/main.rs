mod messenger;

fn main() {
    println!("Welcome to SeyiI!");

    println!("Creating a simple message...");

    let simple_message_count = 1;
    let m: messenger::SimpleMessage = messenger::create_simple_message(simple_message_count, "jose", "mom", "testing message generation...");
    println!("here is the created message {:?}", m);
}
