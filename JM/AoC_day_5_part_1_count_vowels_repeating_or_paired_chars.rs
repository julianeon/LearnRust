use std::fs;

fn get_text(filename: String) -> String {
    let text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    text
}


fn main() {
    
    let filename="nice_strings.txt";
    let long=get_text(filename.to_string());

    let all_chars: Vec<_> = long.chars().collect();
    let mut count=0;
    let mut bad_count=0;
    let mut good_count=0;        

    let mut line_vowels=0;
    let mut last_char='0';        
    for _c in all_chars {
	if _c=='\n' {
	    if (line_vowels >= 3 && bad_count==0 && good_count== 1) {
		count=count+1;
	    }
	    line_vowels=0;
	    bad_count=0;
	    good_count=0;	    	    
	}
	if _c=='a' || _c=='e' || _c=='i' || _c=='o' || _c=='u' {
	    line_vowels = line_vowels+1;
	}
	if (last_char=='a' && _c=='b') || (last_char=='c' && _c=='d') || (last_char=='p' && _c=='q') || (last_char=='x' && _c=='y') {
	    bad_count=1;
	}
	if (last_char==_c) {
	    good_count=1;	    
	}
	last_char=_c;	
    }

    println!("{}",count);
    

}

