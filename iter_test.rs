fn main() {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).unwrap();
  let v: Vec<isize> = tmp.trim().split(" ").map(|n| n.parse().unwrap()).collect();
  one(&v);
  let map: Vec<Vec<isize>> = vec![vec![0; 5]; 10];
  println!("{}", map[0][3]);
}

fn one(vec: &Vec<isize>) {
  two(&vec);
}

fn two(vec: &&Vec<isize>) {
  for i in vec.into_iter() {
    println!("{}", i);
  }
}
