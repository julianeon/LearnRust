use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_text(filename: String) {
    let mut count=0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                //println!("{}", ip);
                let arr=ip.split("x");
		let vec: Vec<&str> = arr.collect();
		let l: i32 = vec[0].parse().unwrap();		
		let w: i32 = vec[1].parse().unwrap();				
		let h: i32 = vec[2].parse().unwrap();						
		let cubic_feet_volume=l*w*h;

		let perimeter_sum=l+l+w+w+h+h;
		let mut largest_side=l;
		if w > largest_side {
		    largest_side=w;
		}
		if h > largest_side {
		    largest_side=h;		    
		}
                let smallest_perimeter_sum=perimeter_sum-(largest_side+largest_side);

		println!("{}-{}-{}: {},{}",l,w,h, cubic_feet_volume,smallest_perimeter_sum);
		count=count+cubic_feet_volume+smallest_perimeter_sum;
            }
        }
    }
    println!("sum is {}",count);
}

fn main() {
    let filename="puzzle2.txt";
    get_text(filename.to_string());
}
