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
		let surface_area=2*l*w+2*w*h+2*l*h;
		let lw=l*w;
		let wh=w*h;
		let lh=l*h;
		let mut smallest_side_area=lw;
		if wh < smallest_side_area {
		    smallest_side_area=wh;
		}
		if lh < smallest_side_area {
		    smallest_side_area=lh;		    
		}
		println!("{}-{}-{}: {},{}",l,w,h, surface_area,smallest_side_area);
		count=count+surface_area+smallest_side_area;
            }
        }
    }
    println!("sum is {}",count);
}

fn main() {
    let filename="puzzle2.txt";
    get_text(filename.to_string());
}
