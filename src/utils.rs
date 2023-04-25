use std::io::Write;

use rand::prelude::*;

pub const ARR_LEN: i32 = 100;

pub fn gen_vec(len: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(len);

    for _ in 0..len {
        vec.push(rng.gen_range(0..1000));
    }

    vec
}

pub fn get_num_input() -> i32 {
    let mut input = String::new();

    std::io::stdout().flush().expect("Error flushing stdout\n");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let choice: i32 = input.trim().parse::<i32>().expect("Input not an integer\n");

    choice
}
