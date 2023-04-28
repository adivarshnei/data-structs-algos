pub fn binsearch(query: i32, vec: &Vec<i32>, len: usize) -> () {
  let idx: i32 = binsearch_util(query, vec, 0, len - 1);

  match idx {
    -1 => println!("{:?} not found", query),
    _ => println!("Found {:?} at index {:?}", query, idx),
  }
}

fn binsearch_util(query: i32, vec: &Vec<i32>, start: usize, end: usize) -> i32 {
  if start > end {
    return -1;
  }

  let mid: usize = (start + end) / 2;

  if query == vec[mid] {
    return mid as i32;
  } else if query < vec[mid] {
    return binsearch_util(query, vec, start, mid);
  } else {
    return binsearch_util(query, vec, mid, end);
  }
}
