use std::fs;

fn get_text(filename: String) -> String {
    let text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    text
}

fn xyadjust (chr: char, x: &mut usize, y: &mut usize) {
	if chr=='>' {
	    *y=*y+1;
	} else if chr=='<' {
            *y=*y-1;	    
	} else if chr=='^' {
            *x=*x-1;	    
	} else if chr=='v' {
            *x=*x+1;	    
	}
}

fn main() {

    let width = 1000;
    let height = 1000;

    let mut board1 = vec![vec![0; width]; height];
    let mut board2 = vec![vec![0; width]; height];    

    let mut x1=500;
    let mut y1=500;
    
    let mut x2=500;
    let mut y2=500;
    
    let filename="puzzle3.txt";
    let long=get_text(filename.to_string());

    let mut count=0;
    board1[x1][y1]=1;
    board2[x2][y2]=1;    
    let all_chars: Vec<_> = long.chars().collect();
    let mut index=0;

    for chr in all_chars {
	if (index % 2)== 0 {
	    xyadjust(chr, &mut x1, &mut y1);
	    board1[x1][y1]=board1[x1][y1]+1;	    
	} else if (index % 2) == 1 {
	    xyadjust(chr, &mut x2, &mut y2);	    
	    board2[x2][y2]=board2[x2][y2]+1;	    
	}
	index=index+1;
    }

    for a in 0..width {
        for b in 0..height {
	    if board1[a][b]!=0 || board2[a][b]!=0 {
		count=count+1;
	    }
        }
    }

    println!("{}",count);
    

}

