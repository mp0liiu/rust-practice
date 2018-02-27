
fn _binary_search(v: &Vec<i8>, head: usize, tail: usize, num: i8) -> Option<usize> {
  let range = (tail - head) / 2;
  let index = range + head;
  if v[index] == num {
    Some(index)
  } else if range == 1 {
    None
  } else if v[index] > num {
    _binary_search(v, head, index, num)
  } else if v[index] < num {
    _binary_search(v, index, tail, num)
  } else {
    None
  }
}

fn binary_search(v: &Vec<i8>, num: i8) -> Option<usize> {
  _binary_search(v, 0, v.len(), num)
}

fn main() {
  let v: Vec<i8> = vec![1, 3, 5, 11, 12, 13, 17, 22, 25, 28];
  let tests = vec![(25, 8), (4, 0), (29, 0)];
  for t in &tests {
    binary_search(&v, t.0).map(|index| {
      println!("{:?}", t);
      assert_eq!(index, t.1);
    });
  }
}

