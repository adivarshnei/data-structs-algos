pub mod searches;
pub mod utils;

fn main() -> () {
    loop {
        println!("1. Searches");
        println!("2. Exit");
        print!("Enter Choice: ");

        let choice: i32 = utils::get_num_input();

        match choice {
            1 => searches(),
            2 => break,
            _ => (),
        }
    }
}

fn searches() -> () {
    println!("1. Linear Search");
    println!("2. Binary Search");
    print!("Enter Choice (Any other number exits): ");
    let choice: i32 = utils::get_num_input();

    match choice == 1 || choice == 2 {
        true => {
            let vec: Vec<i32> = utils::gen_vec(utils::ARR_LEN as usize);
            println!("Element List: ");

            vec[..].into_iter().for_each(|x| {
                print!("{:?} ", x);
            });

            print!("\nEnter element to find: ");
            let query: i32 = utils::get_num_input();

            match choice {
                1 => searches::linear::linsearch(query, &vec, vec.len()),
                2 => println!("Binary Search Not Implemented Yet"),
                _ => (),
            }
        }
        false => (),
    }
}
