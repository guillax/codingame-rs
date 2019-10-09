use std::io;

fn read_enemy_data(input: &io::Stdin) -> (String, i32) {
    let mut input_line = String::new();
    input.read_line(&mut input_line).unwrap();

    let enemy_name = input_line.trim().to_string();

    let mut input_line = String::new();
    input.read_line(&mut input_line).unwrap();

    let enemy_distance = input_line.trim().parse::<i32>().unwrap();

    (enemy_name, enemy_distance)
}

fn main() {
    loop {
        let (enemy_1_name, enemy_1_distance) = read_enemy_data(&io::stdin());
        let (enemy_2_name, enemy_2_distance) = read_enemy_data(&io::stdin());

        let enemy_to_shoot = if enemy_2_distance < enemy_1_distance {
            enemy_2_name
        } else {
            enemy_1_name
        };

        println!("{}", enemy_to_shoot)
    }
}
