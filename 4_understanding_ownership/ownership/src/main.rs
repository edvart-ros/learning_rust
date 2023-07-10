fn main() {

    // string literal(known size), stored on the STACK (fast)
    let s = "hello";
    {
     println!("{s}") //main scope hasnt ended so the variable still exists
    } 
 
    {
    // k is not yet defined
    let k = "k";
    println!("{k}"); // k is defined and in its original scope
    }
    // scope ended so k is not defined here
    // WONT RUN: println!("{k}");

    // exploring the String type, which is stored on the HEAP (slow)
    let _s = String::from("hello"); // define a String(variable size) from a literal
    
    let mut s = String::from("hello"); //mutable version
    s.push_str(", world!");
    println!("{}", s); // hello, world! the string has been changed (mutable, on heap)

    // the variable is still stored in memory...
    // one way to free up the memory that s is holding on the heap:
    {
        let _s = String::from("hello"); // _s is valid in this scope
    } //s is no longer valid, rust **AUTOMATICALLY FREES MEMORY HERE!** using the fn "drop"



    // VARIABLES AND DATA INTERACTING WITH MOVE

    // two variables, both equal to 5, on the stack (immutable, known at compile time)
    let x = 5;  // pushed onto STACK
    let y = x;  // pushed onto STACK


    // two HEAP pointers
    let s1 = String::from("hello"); // s1 points to the string data on THE HEAP
    let s2 = s1; // s2 also points to the same data on the HEAP
    // s2 does NOT copy the data on the heap, it simply *also* stores the ptr, 
    // len, and capacity, where the pointer just points to the *actual* "hello" data on the heap
    // this is way CHEAPER than making COPIES

    // if we now go out of scope. both s1 and s2 need to have their heap memory freed.
    // but they both point to the same memory. this is called "double free error".

    // this is handled by rust simply considering s1 to be invalid when we do "let s2 = s1;".
    // so rust only has to free the data that s2 is pointing to.

    //
    // println!("{}", s1); //THIS WONT WORK SINCE S1 HAS BEEN INVALIDATED
    // the s1 data has been "MOVED" to s2

    println!("{}", s2); //this runs because s2 is still valid (s1 is not)
    // RUST NEVER AUTOMATICALLY MAKES DEEP COPIES


    // forcing a deep copy
    let s1 = String::from("Hello_deep");
    // COPIES ARE EXPENSIVE
    let s2 = s1.clone(); //creates a (deep) copy of s1

    println!("s1 = {}, s2 = {}", s1, s2);



    //  OWNERSHIP AND FUNCTION

    let s = String::from("hello"); //in scope, on the heap
    takes_ownership(s);
    // WE CANNOT USE s ANYMORE since the function took ownership of it (s got moved and then freed)
    //println!("{s}");


    let x = 5; // x is known at compile time. pushed onto STACK
    makes_copy(5); // does not take ownership
    println!("{x}");


    // RETURN VALUES AND SCOPE
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn takes_ownership(some_string: String) { //some_string comes into scope
    println!("{}", some_string);
} //some_string goes out of scope and is dropped, freeing heap memory

fn makes_copy(some_integer: i32) { //some_integer comes into scope
    println!("{}", some_integer);
} //some_integer goes out of scope. nothing is dropped.

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string comes into scope
    some_string //some_string is returned, giving ownership/moved to the caller
}

fn takes_and_gives_back(a_string: String) -> String { //a_string comes into scope.
    // whatever was passed as the argument has been *moved* to this scope/function
    a_string // return back the string to the caller
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); 
    (s, length)
    // in this way we can make sure the caller doesnt lose ownership (by taking and giving back)
    // how do we avoid having to retur back the moved variable? see "references_borrowing"
}