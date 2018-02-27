
fn double_vec(nums: Vec<isize>) -> Vec<isize> {
  nums.iter().map(|x| *x * 2).collect::<Vec<_>>();
}

fn double_vec_ref(nums: &mut Vec<isize>) {
  for mut elem in &mut nums.into_iter().by_ref() {
    *elem *= 2;
  }
}

// boxは所有権付きポインタ
fn double_vec_box(nums: &Vec<isize>) -> Box<Vec<isize>> {
  // *num; // 借用中のポインタはデリファレンスできない
  let new_vec = nums.iter().map(|x| { *x * 2 }).collect::<Vec<_>>();
  Box::new(new_vec)
}

fn print_vec_ref(nums: &Vec<isize>) {
  println!("=======================");
  for (i, elem) in nums.iter().enumerate() {
    println!("i => {}, elem => {}", i, elem);
  }
  println!("");
}

fn main() {

  let mut nums = vec![1; 10];

  nums = double_vec(nums);
  print_vec_ref(&nums);

  double_vec_ref(&mut nums);
  print_vec_ref(&nums);

  // boxをデリファレンスして, その値の参照を渡す
  let nums = *double_vec_box(&nums);
  print_vec_ref(&nums);

}

