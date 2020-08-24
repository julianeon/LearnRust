use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_text(filename: String) {
    let mut good_phrase=0;    
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
		let mut good_one_repeat=0;
		let mut sum_string_chars= 0;
		for c in 2..ip.len() {
		    let d=c+1;
		    let a=c-2;
		    let b=c-1;
		    let ender=&ip[c..d];
		    let beginner=&ip[a..b];
		    let pair=&ip[a..c];
		    if ender==beginner {
			good_one_repeat=1;
		    }
		    let split = ip.split(pair);
		    //print!("{}{}-",beginner,ender);
		    for s in split {
			sum_string_chars=sum_string_chars+s.len();
		    }
		}
		//println!("");		    		
		//println!("{:?}-{}-{}-{}",ip,ip.len(),good_one_repeat,sum_string_chars);
		if good_one_repeat==1 && sum_string_chars!=196 {
		    good_phrase=good_phrase+1;
		}
            }
        }
    }
    println!("good sum is {}",good_phrase);
}

fn main() {
    let filename="nice_strings.txt";
    get_text(filename.to_string());
}
