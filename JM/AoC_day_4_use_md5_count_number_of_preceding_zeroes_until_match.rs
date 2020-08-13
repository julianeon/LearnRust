use md5;

// There was a part 1 and part 2, but the only difference was, you had to find one more zero in part 2.
// So zero = &mdstr[0..5]; changed to zero = &mdstr[0..6]; and if zero=="00000" { changed to if zero=="000000" {
// I just brute forced it - no points for brilliance.
// If no answer appears before the program finishes, then run it again but with a higher range.

fn main() {
    let text="iwrupvqb";
    for x in 1..20000000 {
	let base=text.to_string()+&x.to_string();
	let newbase=base.clone();
	let mdhash = md5::compute(base);    
	let mdstr = format!("{:?}",mdhash);
	let zero = &mdstr[0..6];    
	if zero=="000000" {
	    println!("that's all zeros");
	    println!("{}-{}-{}",newbase,zero,mdstr);	    
	}
    }
}
