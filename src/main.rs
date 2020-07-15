mod messenger;
mod message_buffer;
mod message_storage;

fn main() {
    println!("Welcome to SeyiI!");

    println!("Creating a simple message...");

    let mut simple_message_count = 1;
    let m: messenger::SimpleMessage = messenger::create_simple_message(simple_message_count, "jose", "mom", "First message for MOM testing...");
    println!("here is the created message {:?}", m);

    let mut mb = message_buffer::MessageBuffer::create();
    println!(" Just created a message buffer{:?}", mb);

    let _add_mb_res = match mb.add_to_buffer(&m) {
    	Err(add_err) => println!("There was an error adding message {} to buffer. Details are {}", m.mid, add_err),
    	Ok(add_ok) => println!("{:?}", add_ok),
    };
    println!("revisiting the message buffer {:?}", mb);

    println!("checking borrowing rule for message m {:?}", m);

    simple_message_count += 1;

    let m1: messenger::SimpleMessage = messenger::create_simple_message(simple_message_count, "jose", "mom", "Second message for MOM testing...");
    println!("here is the created message {:?}", m1);

    let _add_mb_res = match mb.add_to_buffer(&m1) {
    	Err(add_err) => println!("There was an error adding message {} to buffer. Details are {}",m1.mid, add_err),
    	Ok(add_ok) => println!("{:?}", add_ok),
    };
    println!("revisiting the message buffer {:?}", mb);

    let _res = match mb.find_message(2) {
    	Some(m3) => println!("here is the retrieved message {:?}", m3),
    	None => println!("No such message"),
    };

    let mut store = message_storage::Storage::create();
    println!("here is the new message store {:?}", store);

    let _res2 = match store.store_message(&m1) {
    	Err(errstr) => println!("An error occurred while storing message {:?}. Error message reads {}", m1, errstr),
    	Ok(resmsg) => println!("The message storage was ok {}", resmsg),
    };
}
