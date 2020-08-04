struct Unit {
    a: String,
    b: String
}

impl Unit {
  fn new(a: String, b: String) -> Self {
    Self { a, b }
  }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value a: {}, value b: {})", self.a, self.b)
    }
}


fn main() {
    let uniter=Unit::new("mark".to_string(),"lowe".to_string());
    println!("{}", uniter);    
    
}
