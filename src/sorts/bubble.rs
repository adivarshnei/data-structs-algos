pub fn bubblesort(vec: &Vec<i32>, len: usize) {
  let mut sorted: Vec<i32> = Vec::new();
  sorted.extend_from_slice(&vec[0..len]);

  for i in 0..len {
    for j in i..len {
      if sorted[i] > sorted[j] {
        let tmp = sorted[i];
        sorted[i] = sorted[j];
        sorted[j] = tmp;

        crate::utils::print_vec(&sorted);
      }
    }
  }
}
