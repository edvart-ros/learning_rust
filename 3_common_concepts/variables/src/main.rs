const _HOUR_IN_SECONDS: u32 = 60*60;

fn main() {

    // Can just reassign values to mut variables
    let mut _x = 5;
    _x = 6;

    // declare immutable (default). 
    // "Shadow" var in new scope. Variable unchanged in 
    // the original scope.
    let y = 10;
    println!("{y}");
    {
        let y = y+1;
        println!("{y}");
    }
    println!("{y}");

    // When we shadow we can change the type.
    // Because we are really just creating a new var with same name.

    let _spaces: &str = "    ";
    let _spaces = _spaces.len(); // spaces: 8, integer
    
        

}
