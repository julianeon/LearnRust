struct Unit {
    a: String,
    b: String
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value a: {}, value b: {})", self.a, self.b)
    }
}

fn main() {
    let test = Unit { a: String::from("joe"), b: String::from("mark") };
    println!("{}", test);    
}
