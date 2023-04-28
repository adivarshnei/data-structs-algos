pub fn linsearch(query: i32, vec: &Vec<i32>, len: usize) -> () {
  for i in 0..len {
    if query == vec[i] {
      println!("Found {:?} at index {:?}", query, i);

      return;
    }
  }

  println!("{:?} not found", query);
}
