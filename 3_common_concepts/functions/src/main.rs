#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    /* 
    some_function();
    let a = 3;
    let b = 4;
    let c = add_ints(a, b);
    println!("{0}", c);
    print_value_of_int(c);

    let float1 = 0.1;
    let float2 = 0.3;
    
    let float3 = add_floats(float1, float2);
    println!("{float3}");
    */

    // EXPRESSIONS VS STATEMENTS
    let y = 6; //statement

    //this is an expression
    // 3 + 2;


    // this is an expression containing two statements!
    let y = {
        let x = 3;
        let z = 5;
        x*z
    };
    println!("{y}");
}


fn add_floats(a: f64, b: f64) -> f64 {
a + b //this is what the function returns. no semicolon and last in the function returns implicitly!
}
    
fn print_value_of_int(x: i32) {
println!("the value of the integer is {x}");
}

fn some_function() {
println!("You called 'some function'!");
}

fn add_ints(a: i32, b: i32) -> i32 {
a + b
}
