use std::io::{self, BufRead};

fn main() {
    loop {
        let reader = io::BufReader::new(io::stdin());
        let max_height_idx = reader
            .lines()
            .map(|res| res.unwrap().trim().parse::<i32>().unwrap())
            .take(8)
            .enumerate()
            .max_by_key(|&(_, height)| height)
            .unwrap()
            .0;
        println!("{}", max_height_idx);
    }
}
