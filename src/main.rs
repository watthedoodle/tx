mod figlet;

use crate::figlet::FIGfont;

fn main() {
    let sf = FIGfont::standard().unwrap();
    let io = sf.convert("Hello World");
    println!("{}", io.unwrap());
}

fn link_in_bio(s: &str) -> String {
    // ░L░I░N░К░ ░I░N░ В░I░О░
    let mut genned = String::new();
    for c in s.chars() {
        let local = c.clone().to_string();
        if c != ' ' {
            genned.push_str(&"░");
            genned.push_str(&local);
        } else {
            genned.push_str(&" ");
        }
    }
    genned
}
