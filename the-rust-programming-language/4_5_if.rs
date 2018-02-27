
fn read_line() -> String {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).unwrap();
  tmp.trim().to_string()
}

fn main() {

  let x = 5;
  if x == 5 {
    println!("x is {}", x);
  }

  let num: isize = read_line().parse().unwrap();
  // 値を返す
  let x = if num == 5 {
    "num is 5."
  } else if num == 10 {
    "num is 5*2"
  } else {
    "num is not 5."
  };
  println!("{}", x);

  // elseのないif文は常に()を返す
  // というか型の不一致でちゃんと使えないし
  let hum = if num == 1 { () };
  assert_eq!(hum, ());

}
