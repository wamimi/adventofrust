// Blitzen and Snowball’s Unproductive Debate
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let s1_len = s1.trim().chars().count();
    let s2_len = s2.trim().chars().count();
    
    if s1_len == s2_len {
        None
    } else if s1_len > s2_len {
        Some(s1)
    } else {
        Some(s2)
    }
 }