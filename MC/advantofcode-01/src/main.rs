use std::fs;

fn calculate_fuel(val: i32) -> i32 {
    return val;
}

fn main() {
    let contents = fs::read_to_string("./src/input")
        .expect("Something went wrong reading the file");

    let split = contents.split("\n");
    let mut total = 0; 
    for value in split {
        if !value.is_empty() {
            total = total + calculate_fuel(value.parse::<i32>().unwrap());
        }
    }
    println!("Total: {}", total);
}
