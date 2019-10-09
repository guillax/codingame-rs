use std::io;

fn read_montain_data(input: &io::Stdin) -> i32 {
    let mut input_line = String::new();
    input.read_line(&mut input_line).unwrap();

    input_line.trim().parse::<i32>().unwrap()
}

fn main() {
    loop {
        let mut index = 0;
        let mut max_height = 0;

        for i in 0..8 as usize {
            let height = read_montain_data(&io::stdin());
            if height > max_height {
                index = i;
                max_height = height;
            }
        }

        println!("{}", index);
    }
}
