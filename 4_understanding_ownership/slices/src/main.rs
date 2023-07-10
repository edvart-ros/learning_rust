#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5]; // a reference to "hello"
    let hello = &s[..5];

    let world = &s[6..11]; // a reference to "world"
    let world = &s[6..];

    let first = first_word(&s);
    println!("{first}");

    // slicing the whole thing is equivalent to just referencing the value
    let s1 = &s[..];
    let s2 = &s;

    if s1 == s2 {
        println!("success!");
    }
}

// returns the index of the end of the word
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // find the first space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
    
}