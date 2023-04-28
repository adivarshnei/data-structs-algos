pub mod searches;
pub mod sorts;
pub mod utils;

fn main() -> () {
  loop {
    println!("1. Searches");
    println!("2. Sorts");
    println!("3. Exit");
    print!("Enter Choice: ");

    let choice: i32 = utils::get_num_input();

    match choice {
      1 => searches(),
      2 => sorts(),
      3 => break,
      _ => (),
    }
  }
}

fn sorts() -> () {
  println!("\nSorts");
  println!("1. Bubble Sort");
  println!("2. Selection Sort");
  println!("3. Insertion Sort");
  println!("4. Merge Sort");
  println!("5. Quick Sort");
  println!("6. Heap Sort");
  println!("7. Counting Sort");
  println!("8. Radix Sort");
  println!("9. Bucket Sort");
  println!("10. Bingo Sort");
  println!("11. ShellSort");
  println!("12. TimSort");
  println!("13. Comb Sort");
  println!("14. Pigeonhole Sort");
  println!("15. Cycle Sort");
  println!("16. Cocktail Sort");
  println!("17. Strand Sort");
  println!("18. Bitonic Sort");
  println!("19. Pancake Sort");
  println!("20. BogoSort");
  println!("21. Gnome Sort");
  println!("22. Sleep Sort");
  println!("23. Structure Sort");
  println!("24. Stooge Sort");
  println!("25. Tag Sort");
  println!("26. Tree Sort");
  println!("27. Brick Sort");
  println!("28. 3-way Merge Sort");
  print!("Enter Choice (Any other number exits): ");
  let choice: i32 = utils::get_num_input();

  match choice {
    1..=28 => {
      let vec: Vec<i32> = utils::gen_vec(utils::ARR_LEN as usize);
      utils::print_vec(&vec);

      match choice {
        1 => sorts::bubble::bubblesort(&vec, vec.len()),
        _ => (),
      }
    }
    _ => (),
  }
}

fn searches() -> () {
  println!("\nSearches");
  println!("1. Linear Search");
  println!("2. Binary Search");
  print!("Enter Choice (Any other number exits): ");
  let choice: i32 = utils::get_num_input();

  match choice {
    1..=2 => {
      let vec: Vec<i32> = utils::gen_sorted_vec(utils::ARR_LEN as usize);
      println!("Element List: ");

      utils::print_vec(&vec);

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
