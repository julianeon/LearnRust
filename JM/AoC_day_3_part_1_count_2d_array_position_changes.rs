use std::fs;

fn get_text(filename: String) -> String {
    let text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    text
}

fn main() {

    let width = 1000;
    let height = 1000;

    let mut board = vec![vec![0; width]; height];

    let mut x=500;
    let mut y=500;
    
    let filename="puzzle3.txt";
    let long=get_text(filename.to_string());

    let mut count=0;
    board[x][y]=1;
    let all_chars: Vec<_> = long.chars().collect();
    for chr in all_chars {
	if chr=='>' {
	    x=x+1;
	} else if chr=='<' {
            x=x-1;	    
	} else if chr=='^' {
            y=y+1;	    
	} else if chr=='v' {
            y=y-1;	    
	}
	board[x][y]=board[x][y]+1;
    }
    //for line in &board {
	//println!("{:?}",line);
    //}
    for a in 0..1000 {
        for b in 0..1000 {
	    if board[a][b]!=0 {
		count=count+1;
	    }
		
        }
    }
    println!("{}",count);
}


