fn main() {
    let instruction: Vec<String> = std::env::args().collect();
    let instruction: &String = &instruction[1];

    println!("{}", santa(instruction));
}

fn santa(instruction: &String) -> i32 {
    // if '(' up else if ')' down
    let mut floor: i32 = 0;
    for paren in instruction.chars() {
        println!("{}", paren);
        match paren {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!(),
        }
    }
    floor
}

// #[cfg(test)]
mod test {
    use super::santa;
    #[test]
    fn example() {
        assert!(santa(&"(())".to_string()) == 0);
        assert!(santa(&"()()".to_string()) == 0);
        assert!(santa(&"(((".to_string()) == 3);
        assert!(santa(&"(()(()(".to_string()) == 3);
        assert!(santa(&"))(((((".to_string()) == 3);
        assert!(santa(&"())".to_string()) == -1);
        assert!(santa(&"))(".to_string()) == -1);
        assert!(santa(&")))".to_string()) == -3);
        assert!(santa(&")())())".to_string()) == -3);
    }
    #[test]
    fn a() {
        assert!(santa(&"".to_string()) == 0);
    }
    #[test]
    #[should_panic]
    fn b() {
        santa(&"{}".to_string());
    }
}
