use std::io;
use std::str::FromStr;

fn read_line<T>(input: &io::Stdin) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut input_line = String::new();
    input.read_line(&mut input_line).unwrap();

    input_line.trim().parse::<T>().unwrap()
}

fn main() {
    let temperatures_count = read_line::<i32>(&io::stdin());
    if temperatures_count == 0 {
        println!("0");
        std::process::exit(0);
    }

    let mut temperatures_str = String::new();
    io::stdin().read_line(&mut temperatures_str).unwrap();

    let mut nearest_to_zero = 5526_i32;
    for temperature_str in temperatures_str.split_whitespace() {
        let t = temperature_str.trim().parse::<i32>().unwrap();
        if t.abs() < nearest_to_zero.abs() {
            nearest_to_zero = t;
        } else if t.abs() == nearest_to_zero.abs() && t > nearest_to_zero {
            nearest_to_zero = t;
        }
    }

    println!("{}", nearest_to_zero);
}
