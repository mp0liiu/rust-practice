fn main() {

  // bool
  let x: bool = true;
  let y: bool = false;
  if x {
    println!("hell")
  }
  if y {
    println!("this is y")
  }

  // char
  let x = 'x';
  let heart = '♥';
  assert_eq!(std::mem::size_of_val(&x), 4);
  assert_eq!(std::mem::size_of_val(&heart), 4);

  let v = vec!['潮', '田', '渚'];
  assert_eq!(12, v.len() * std::mem::size_of::<char>());
  for c in &v {
    println!("{}", c)
  }

  let s = String::from("hello");
  assert_eq!(5, s.len());

  let s = String::from("♥");
  assert_eq!(3, s.len());  // ?
  assert_eq!(24, std::mem::size_of_val(&s));
  let s = String::from("love : ♥");
  let v: Vec<_> = s.chars().collect();
  assert_eq!(10, s.len()); // ?
  assert_eq!(8, v.len());  // ?

  let c = '1';
  assert!( c.is_digit(10) );
  let c = '9';
  assert!( c.is_digit(19) );
  assert!( 'f'.is_digit(16) );

  let s = '来'.escape_unicode().to_string();
  println!("{}", s);
  assert!('A'.is_alphabetic());
  assert!('0'.is_numeric());
  assert_eq!('c'.to_uppercase().next().unwrap(), 'C');

  // String と &str は to_string() と as_str() で相互変換可能 (コピーが作られる)
  let chars = "hello";
  let s = chars.to_string();
  let chars2 = s.as_str();
  let s2 = chars2.to_string();
  assert_eq!(chars, chars2);
  assert_eq!(s, s2);

  // 数値型

  // 符号有り整数型
  let x: i8 = 9;
  assert_eq!(std::mem::size_of_val(&x), 1);
  let x: i16 = -20;
  assert_eq!(std::mem::size_of_val(&x), 2);
  let x: i32 = -5;
  assert_eq!(std::mem::size_of_val(&x), 4);
  let x: i64 = -55;
  assert_eq!(std::mem::size_of_val(&x), 8);
  // コンピュータのマシンのポインタのサイズに依存する型 このPCは64bit CPUなので64
  let x: isize = 50;
  assert_eq!(std::mem::size_of_val(&x), 8);

  // 符号なし整数型
  let x: u8 = 9;
  assert_eq!(std::mem::size_of_val(&x), 1);
  let x: u16 = 20;
  assert_eq!(std::mem::size_of_val(&x), 2);
  let x: u32 = 5;
  assert_eq!(std::mem::size_of_val(&x), 4);
  let x: u64 = 55;
  assert_eq!(std::mem::size_of_val(&x), 8);
  // コンピュータのマシンのポインタのサイズに依存する型 このPCは64bit CPUなので64
  let x: usize = 50;
  assert_eq!(std::mem::size_of_val(&x), 8);

  // 浮動小数点型
  let x: f32 = 3.5;
  assert_eq!(std::mem::size_of_val(&x), 4);
  let x: f64 = -26.92932;
  assert_eq!(std::mem::size_of_val(&x), 8);
  
  // 固定長配列
  let a: [i8; 3] = [1, 2, 3];
  assert_eq!(a, [1, 2, 3]);
  assert_eq!(std::mem::size_of_val(&a), 3);
  assert_eq!(a[1], 2);

  // 固定長配列のindexはusize型でないといけない
  let index: usize = 0;
  assert_eq!(a[index], 1);

  let a: [i64; 3] = [1, 2, 3];
  assert_eq!(std::mem::size_of_val(&a), 24);

  let a: [i16; 5] = [0; 5];
  assert_eq!(a, [0, 0, 0, 0, 0]);
  assert_eq!(std::mem::size_of_val(&a), 10);
  assert_eq!(a.len(), 5);

  let a = ['赤', '羽', '業'];
  assert_eq!(std::mem::size_of_val(&a), 12);
  let mut s = String::new();
  for &c in &a {
    s.push(c);
  }
  println!("{}", s);

  let a = ["hirakata", "-", "papark"];
  let mut s = String::from("");
  for &t in &a {
    s.push_str(t);
  }
  println!("{}", s);
  println!("{:?}", a);

  // slice
  let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
  let all = &a[..];
  assert_eq!(all.len(), 10);
  let parts = &a[1 .. 3];
  assert_eq!(parts.len(), 2);
  assert_eq!(parts, [2, 3]);

  // tuple
  let x = (1, "垣根帝督");
  assert_eq!(x.0, 1);
  assert_eq!(x.1, "垣根帝督");

  let mut x: (char, &str) = ('C', "MP18");
  assert_eq!(x.1, "MP18");
  let y = ('D', "Automatico");
  x = y;
  assert_eq!(x.1, "Automatico");

  // 一個のタプル
  let x = (111,);
  let y = (110);
  assert_eq!(x.0, 111);
  assert_eq!(y, 110);
  
  // 関数
  fn foo(x: i8) -> i8 { x }
  let x: fn(i8) -> i8 = foo;
  x(10);

}

