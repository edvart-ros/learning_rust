fn main() {
    let _x = 4.0; //f64 (default)
    let _y: f32 = 4.0; //f32

    let _z = _x*_y;

    println!("{}", 6/-1); //prints -6
    println!("{}", 6/7); //prints 0 (truncated since int/int = int)!

    let remainder = 10%3; //modulus
    println!("{remainder}");


    let t = true; // rust infers bool type
    let f: bool = false;
    println!("{t}, {f}"); // out: "true, false"


    // chars, SINGLE QUOTES
    let _z = 'z'; // inferred 
    let _z: char = 'z'; // explicit
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");


    // Tuples
    let tup: (u8, i32, f64) = (12, -300, 6.789);
    let (x, y, z) = tup; // assign the elements in tup to variables
    // equivalents:
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    println!("{x}, {y}, {z}");




}
