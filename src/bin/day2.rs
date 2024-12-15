// Unsolved challenge Code
fn main() {
    // Uses .clone() to create a copy without losing ownership
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(gift_message.clone()); 
    println!("{}", gift_message);
 }
 
 //This Approach is suboptimal. why?
 // Uses .clone(), which creates unnecessary memory allocation
 // Inefficient for larger strings
 // Doesn't leverage rust's borrowing system
 
 // Solved solution
 fn main() {
    // Uses reference (&) to borrow the string without transferring ownership
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(&gift_message);     // Passing a reference
    println!("{}", gift_message);                 // Original string still accessible
 }
 
 // Updated function to accept a reference instead of owned String
 fn attach_message_to_present(message: &str) {    
    println!("The present now has this message: {}", &message);
 }