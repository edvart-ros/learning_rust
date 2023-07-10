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







}
