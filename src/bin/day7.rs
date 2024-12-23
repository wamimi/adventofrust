// the great log hunt
// 1. Finish the implementation of LogQuery
pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
 }
 // Create a public associated function named `new()` that will take a reference to a vector of strings
 impl<'a> LogQuery<'a> {
     pub fn new(logs: &'a Vec<String>) -> Self {
         Self { logs }
     }
 //3. Create a public method named `search` that accepts a string slice and finds it from the logs and
 //    returns a vector of references to those logs.
     pub fn search(&self, keyword: &str) -> Vec<&'a String> {
         self.logs
             .iter()
             .filter(|log| log.contains(keyword))
             .collect()
     }
 }