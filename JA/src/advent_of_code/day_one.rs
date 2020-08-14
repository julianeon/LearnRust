use std::fs::File;
use std::io::{prelude::*, Error, ErrorKind, Result as IOResult};

fn read_file(contents: &mut String) -> IOResult<()> {
    let mut file = File::open("files/day_one")?;
    file.read_to_string(contents)?;
    Ok(())
}
pub fn count_floors() -> IOResult<i32> {
    let mut contents = String::new();
    let mut count = 0;
    read_file(&mut contents)?;
    while let Some(c) = contents.pop() {
        if c == '(' {
            count += 1;
        } else {
            count -= 1;
        }
    }
    Ok(count)
}

pub fn find_floor_minus_one() -> IOResult<usize> {
    let mut contents = String::new();
    let mut idx = None;
    read_file(&mut contents)?;
    let mut count = 0;
    for (i, c) in contents.chars().enumerate() {
        if c == '(' {
            count += 1;
        } else {
            count -= 1;
        }

        if count == -1 {
            idx = Some(i+1);
            break;
        }
    }

    idx.ok_or(Error::new(
        ErrorKind::InvalidInput,
        "input does not go to floor -1",
    ))
}
