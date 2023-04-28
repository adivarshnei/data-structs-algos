use std::io::Write;

use rand::prelude::*;

pub const ARR_LEN: i32 = 20;

pub fn gen_vec(len: usize) -> Vec<i32> {
  let mut rng: ThreadRng = rand::thread_rng();
  let mut vec: Vec<i32> = Vec::with_capacity(len);

  for _ in 0..len {
    let elem: i32 = rng.gen_range(1..50);
    vec.push(elem);
  }

  vec
}

pub fn gen_sorted_vec(len: usize) -> Vec<i32> {
  let mut rng: ThreadRng = rand::thread_rng();
  let mut vec: Vec<i32> = Vec::with_capacity(len);

  let mut prev: i32 = 0;

  for _ in 0..len {
    let elem: i32 = rng.gen_range(1..50) + prev;
    vec.push(elem);
    prev = elem;
  }

  vec
}

pub fn print_vec(vec: &Vec<i32>) -> () {
  vec[..].into_iter().for_each(|x| {
    print!("{} ", x);
  });

  println!();
}

pub fn get_num_input() -> i32 {
  let mut input: String = String::new();

  std::io::stdout().flush().expect("Error flushing stdout\n");

  while input.is_empty() || !input.trim().parse::<i32>().is_ok() {
    std::io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
  }

  let choice: i32 =
    input.trim().parse::<i32>().expect("Input not an integer\n");

  choice
}
