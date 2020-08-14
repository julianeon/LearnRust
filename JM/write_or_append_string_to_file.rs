use std::fs::File;
use std::path::Path;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;

fn rando() -> String {
    let mut rng = rand::thread_rng();
    format!("{}-", rng.gen_range(1, 100))
}

fn write_string_to_file(fname: &str, line: &str) {
    let path = Path::new(fname);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(line.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => (),
    }
}

fn append_string_to_file(aname: &str, astring: &str) {
    let mut file = OpenOptions::new().append(true).open(aname).expect("cannot open file");
    file.write_all(astring.as_bytes()).expect("write failed");
}

fn main() {
    let mut s=String::new();
    for _ in 0..6 {
	let result=rando();	
	s=String::new()+&s+&result;
    }
    let len = s.len();
    let slice = &s[0..len-1];
    let line = format!("{}\n",slice);

    let fname = "number.txt";
    let aname = "guess.txt";
    
    //println!("{}",slice);
    //append_string_to_file(aname,&line);
    write_string_to_file(fname,&line);    
}




