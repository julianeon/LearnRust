use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input").expect("Something went wrong reading the file");
    //let contents:&str = "(())";
    let mut floor:i32 = 0;
    let mut i:u32 = 1;
    let mut found_it:bool = false;
    for c in contents.chars() { 
        if c.to_string() == "(" {
            floor = floor + 1;
        } else if c.to_string() == ")" {
            floor = floor - 1;
        }
        if floor == -1 && found_it == false {
            found_it = true;
        }
        if found_it == false {
            i = i + 1;
        } 
    }
    println!("First Basement Position: {}", i);
}
