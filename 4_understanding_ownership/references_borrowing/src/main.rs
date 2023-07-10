fn main() {
    // avoid giving away ownership by using references?

    let s1 = String::from("12345");
    let len = calculate_length(&s1);
    println!("{}", len); // out: 5

    // WE CANNOT CHANGE A VALUE THROUGH A REFERENCE!!
    //let s2 = String::from("hello");
    //change(&s2); // attempt to change what the function is borrowing (only a reference)

    // MUTABLE REFERENCES

    let mut s2 = String::from("hello");
    println!("{}", s2); // out: hello
    change(&mut s2); // change() changes the actual value of s2
    println!("{s2}"); // out: hello, world!

    // DANGLING REFERENCES
    //let reference_to_nothing = dangle();
    // this causes compiler error, because we cant return a reference to something whose
    // value has been deallocated from memory!!

    let s = no_dangle();
    println!("{}", s);
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

/*

fn dangle() -> &String {
    let s = String::from("hello"); //s is now in scope
    &s //returns a reference to the variable created in this scope
} //s goes out of scope, and memory is dropped. but what about the reference??
// Danger!
*/


fn calculate_length(s: &String) -> usize { // gets a reference only
    s.len() //returns the length
} //the ultimate data of s is not dropped since its only a reference


/* 
fn change(s: &String) {
    s.push_str(", world!"); //this is not allowed since s is only a reference here
    // out:  `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}
*/

fn change(s: &mut String) {
    s.push_str(", world!");
}