use rand::seq::SliceRandom; 

fn either(samples: &Vec<&str>) -> () {
    let sample: Vec<_> = samples
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();
    println!("{:?}", sample);
}


fn main() {
    let duo = vec!["red", "blue"];    
    either(&duo);
}
