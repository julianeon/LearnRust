use std::fs;

fn get_text(filename: String) -> String {
    let text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    text
}

fn main() {
    let filename="example.txt";
    let long=get_text(filename.to_string());

    let mut count=0;
    let all_chars: Vec<_> = long.chars().collect();
    for chr in all_chars {
	if chr=='(' {
	    count=count+1;
	} else {
	    count=count-1;
	}
    }
    println!("{}", count);    
}
