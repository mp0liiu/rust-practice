
fn main() {

  // 無限ループ文
  let mut count = 0;
  loop {
    count += 1;
    if count == 5 {
      break;
    }
    println!("loop {}", count);
  };

  // while文
  let mut x = 1;
  let mut done = false;
  while !done {
    x = x * 3 + 1;;
    println!("x => {}", x);
    if x % 5 == 0 {
      done = true;
    }
  }

  // for文
  let ary = ['茅', '野', 'カ', 'エ', 'デ'];
  // 暗黙的に .into_iter() が呼ばれている
  for c in &ary {
    // c: &char
    print!("{}", *c);
  }
  for c in &ary {
    // しかし、自動的にデリファレンスされるので*なくてもいける
    print!("{}", c); // ok
  }
  println!("");

  let doubles: Vec<_> = ary.iter().map(|x| { std::mem::size_of_val(x) * 2 }).collect();
  for num in &doubles {
    print!("{}", *num);
  }
  println!("");

  // c style like
  for i in 0 .. 10 {
    // i: {integer}
    print!("{}, ", i);
  }
  println!("");

  // for文の中で何回目のくり返しか知りたい
  for (i, elem) in doubles.iter().enumerate() {
    // i = 回数
    // i: usize, elem: &usize
    println!("i => {}, elem => {}", i, *elem);
  }

  // continue
  for n in 0 .. 50 {
    if n % 2 == 0 {
      continue;
    } else if n % 3 == 0 {
      println!("333333333333333#");
    } else if n == 39 {
      break;
    }
  }

  // ループラベル
  'outer: for x in 0 .. 10 {
    'inner: for y in 0 .. 10 {
      if x % 2 == 0 { continue 'outer } // 外側のループを継続
      if y % 2 == 0 { continue 'inner } // 内側のループを継続
      println!("x => {}, y => {}", x, y);
    }
  }

}
