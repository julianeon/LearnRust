fn main() {
    let long="())()(()....(you get the idea)";
    let mut count=0;
    let my_chars: Vec<_> = long.chars().collect();
    for n in my_chars {
	if n=='(' {
	    count=count+1;
	} else {
	    count=count-1;
	}
    }
    println!("{}", count);    
}
