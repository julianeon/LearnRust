use rand::seq::IteratorRandom; 
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use rand::seq::SliceRandom;

fn print_either(samples: &Vec<String>) -> () {
    let sample: Vec<_> = samples
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();
    println!("{}", &sample[0]);
}

fn find_word(filename: &str) -> String {
    let f = File::open(filename)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", filename, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}

fn select_from_duo(filea: &str, fileb: &str) {
    let x=find_word(filea);
    let y=find_word(fileb);
    let samples=vec![x,y];
    print_either(&samples);
}

fn main() {
    select_from_duo("animate.txt","inanimate.txt");
    select_from_duo("day.txt","night.txt");
    select_from_duo("dry.txt","wet.txt");        
}
