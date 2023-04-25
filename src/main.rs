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
    println!("\nSearches");
    println!("1. Linear Search");
    println!("2. Binary Search");
    print!("Enter Choice (Any other number exits): ");
    let choice: i32 = utils::get_num_input();

    match choice {
        1 | 2 => {
            match choice {
                1 => println!("\nLinear Search\n"),
                2 => println!("\nBinary Search\n"),
                _ => (),
            }

            let vec: Vec<i32> = utils::gen_vec(utils::ARR_LEN as usize);
            println!("Element List: ");

            vec[..].into_iter().for_each(|x| {
                print!("{:?} ", x);
            });

            print!("\nEnter element to find: ");
            let query: i32 = utils::get_num_input();

            match choice {
                1 => searches::linear::linsearch(query, &vec, vec.len()),
                2 => searches::binary::binsearch(query, &vec, vec.len()),
                _ => (),
            }
        }

        _ => (),
    }
}
