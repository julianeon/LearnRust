use std::fs::File;
use std::io::{prelude::*, BufReader, Result as IOResult};

fn read_file() -> IOResult<BufReader<File>> {
    let file = File::open("files/day_two")?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub fn total_wrapping_paper_size() -> IOResult<u32> {
    let reader = read_file()?;

    let total_surface_area = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut split_line = line
                .split('x')
                .map(|val| val.parse::<u32>().unwrap())
                .peekable();

            let mut first = None;

            let mut min = u32::MAX;
            let mut surface_area = 0;

            while let Some(num) = split_line.next() {
                let area;
                if first == None {
                    first = Some(num);
                }
                if let Some(next_num) = split_line.peek() {
                    area = num * next_num;
                } else {
                    area = num * first.unwrap();
                }
                if area < min {
                    min = area
                }

                surface_area = surface_area + (area * 2)
            }
            surface_area + min
        })
        .sum::<u32>();

    Ok(total_surface_area)
}

pub fn total_ribbon_size() -> IOResult<u32> {
    let reader = read_file()?;

    let total_ribbon_length = reader
        .lines()
        .map(|line| {
            let line = line?;
            let mut values = line
                .split('x')
                .map(|val| val.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            values.sort();
            let ribbon_length =
                values.iter().fold(1, |a, b| a * b) + (values[0] * 2) + (values[1] * 2);
            Ok(ribbon_length)
        })
        .map(|len: IOResult<u32>| len.unwrap())
        .sum();

    Ok(total_ribbon_length)
}
