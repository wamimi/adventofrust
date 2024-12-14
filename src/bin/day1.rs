// Original Unsolved Challenge Code
fn main() {    
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");    
    attach_message_to_present(gift_message);     
    println!("{}", gift_message);
} 

fn attach_message_to_present(message: String) {    
    println!("The present now has this message: {}", message);
}

// Why This Fails:
// - The original code transfers ownership of gift_message
// - After passing to attach_message_to_present(), ~
//   gift_message can no longer be used

// Solved Solution
fn main() {
    // Use .clone() to create a copy without losing ownership
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(gift_message.clone()); 
    println!("{}", gift_message);
}