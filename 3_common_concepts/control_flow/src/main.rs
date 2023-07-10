fn main() {
    let a = 5;
    let b = 9;

    compare_ints(a, b);

    for n in 1..=100 {
        fizz_buzz(n)
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut count = 0;

    let result = loop {
        if count == 10 {
            break count*2;
        }
        count += 1;
    };
    println!("the result is {result}");

    let mut number = 0;
    while number < 5 {
        println!("{number}");
        number += 1;
    }

    let start = 10;
    let end = 0;
    println!("Counting down from {start} to {end}!");
    for i in (0..=10).rev() {
        println!("{i}");
    }
    println!("liftoff!");

    let x = fib(4);
    println!("{x}");

    

}


fn fib(n: i32) -> i32 {
    let mut result = 1;
    for i in (1..=n).rev() {
        result = result*i;
    }
    result  
}


fn compare_ints(a:i32, b:i32) {
    if a < b {
        println!("{a} is less than {b}!");
    } else if a == b {
        println!("{a} is equal to {b}");
    } else{
        println!("{a} is greater than {b}");
    }
}


fn fizz_buzz(number:i32) { 
    if number % 15 == 0 {
        println!("fizzbuzz");
    } else if number % 5 == 0 {
        println!("buzz");
    } else if number % 3 == 0 {
        println!("fizz");
    } else {
        println!("{number}");
    }
}