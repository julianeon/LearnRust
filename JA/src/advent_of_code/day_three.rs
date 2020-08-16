use std::fs::File;
use std::io::{prelude::*, BufReader, Result as IOResult};
use std::collections::HashSet;


fn read_file() -> IOResult<BufReader<File>> {
    let file = File::open("files/day_three")?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn calculate_houses_one_present() {

}