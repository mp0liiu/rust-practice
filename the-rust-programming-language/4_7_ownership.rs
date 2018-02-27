
// 仮引数は引数から所有権を譲られる
fn take(v: Vec<i32>) {
  println!("vec 0 {} ", v[0]);
}

// 引数からもらった所有権をまた譲るには, その変数を返却値にすれば良い
fn reverse(v: Vec<i32>) -> Vec<i32> {
  println!("vec 0 {} ", v[0]);
  v
}

fn main () {

  // 変数束縛は、束縛されているものの所有権を持つ
  // ここでは、vがスコープに入るとき, 新しいVec<i32>が作られ, ヒープにこのベクタの領域を割り当てる
  // 変数がスコープから外れた時, 割り当てたメモリを開放する
  let v: Vec<i32> = vec![1, 2, 3];

  // 1つのデータリソースに対する束縛は, プログラムの中で1つしか存在しない
  // 違う変数に束縛しなおした場合は, 元の変数は所有権を失い, 元の変数からは元のデータを使うことができなくなる
  // 代わりに束縛しなおした変数からデータを使うことができるようになる(ムーブセマンティクス)
  let v2 = v;
  // println!("v[0] is {}", v[0]); // -> is error
  println!("v2[0] is {}", v2[0]); // -> ok

  // 所有権を受け取る関数を定義して、引数として何かを渡した後でそれをもう一度使おうとすると、同じエラーが起きる
  // (所有権を関数の仮引数に渡したので, 引数の方の所有権は喪失)
  take(v2);
  // println!("v2[0] is {}", v2[0]); // -> error

  let v3: Vec<i32> = vec![4, 5, 6];
  println!(" v3[2] is {}", v3[2]);
  let v4 = reverse(v3);
  println!(" v4[2] is {}", v4[2]);
  // println!(" v3[2] is {}", v3[2]); // -> error

  // Copy型
  type_copy();

  // もし, 複数の束縛の所有権を渡し, 渡した所有権を全て返してもらう関数を書くとこうなる
  let s1 = Student { name: "dekiruyo".to_string(), age: 33, score: 100 };
  let s2 = Student { name: "One_More_Dead".to_string(), age: 28, score: 90 };
  let s3 = Student { name: "Vemeru".to_string(), age: 25, score: 66 };
  let (s1, s2, s3, sum) = student_hogehoge(s1, s2, s3);
  println!("score sum {}", sum);
  assert_eq!(s1.name.as_str(), "dekiruyo");
  assert_eq!(s2.name.as_str(), "One_More_Dead");
  assert_eq!(s3.name.as_str(), "Vemeru");

}

fn student_hogehoge(s1: Student, s2: Student, s3: Student) -> (Student, Student, Student, i32) {
  println!("{} {} {}", s1.name, s2.name, s3.name);

  // ちなみにこうやってベクタの中の要素に束縛をそのまま使うと, 所有権もベクタの中に移動する
  // let mut students = vec![s1, s2, s3];
  let mut students = vec![s1, s2, s3];
  let students2 = students.clone(); // student が clone できるので, データのコピーは可能
  students.sort_by(|a, b| { a.score.cmp(&b.score) });
  println!("top score : {}", students[2].score);
  students.sort_by(|a, b| { a.age.cmp(&b.age) });
  println!("top age : {}", students[2].age);
  let sum = students.iter().fold(0, |sum, student| { sum + student.score as i32 });
  let mut iter = students2.into_iter(); // イテレータにベクタの要素の所有権が譲渡される

  /* ベクタの中の所有権は, 添字からの参照では移動できない
   * return (students[0], students[1], students[2], sum); // -> error : cannot move out of indexed content
   * 代わりにどうするか?
   * 1. ベクタには構造体の参照を渡し, 参照のベクタを作る
   *   この場合, 借用中の要素は返却できなくなるため, 
   *   もとの構造体を返したいときはベクタで操作するときのみスコープを狭める必要がある
   * 2. ベクタのイテレータをつくり, into_iter() で要素の所有権をイテレータに譲渡,
   *   イテレータから値を取り出し値と所有権を返却する(今の方法)
  */


  // 所有権と関数の結果を返す
  (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), sum)
}

// Copy型について

/* Copy trait 実装していない型 */
struct Student {
  name: String,
  age: i8,
  score: i8,
}

// clone トレイトで明示的にclone呼んでコピー作らせることはできる
impl Clone for Student {
  fn clone(&self) -> Student {
    Student { name: self.name.clone(), age: self.age, score: self.score }
  }
}

/* Copy trait 実装 */
struct Point {
  x: i32,
  y: i32,
}

impl Copy for Point {}

impl Clone for Point {
  fn clone(&self) -> Point { *self }
}
/* --- */

/* Copy trait 実装2 */
#[derive(Copy, Clone)]
struct Ball {
  x: i32,
  y: i32,
  degree: i16,
}

fn double(n: i32) -> i32 {
  n * 2
}

fn change_truth(x: bool) -> bool {
  !x
}

fn type_copy() {

  // Copy型について
  // 所有権が他の束縛に譲渡されたとき, もとの束縛は利用できなくなりますが,
  // Copyトレイトを実装した型の値は, 他の束縛に値を束縛した時, 元の値の完全なコピーを束縛する
  // 全てのプリミティブ型は, Copyトレイトを実装している
  let n: i32  = 1;
  let n2: i32 = n;
  assert_eq!(n, n2); // ok n2にはnの値の完全なコピーが束縛されている

  let p = Student { name: "liiu".to_string(), age: 21, score: 0 };
  let p2 = p;
  // println!("{}", p.name); // これはCopyトレイトを実装していない型なのでエラー

  let point = Point { x: 5, y: 3 };
  let point2 = point; // Point はCopyトレイトを実装しているためok
  assert_eq!(point.x, point2.x);
  assert_eq!(point.y, point2.y);

  let ball = Ball { x: 0, y: 0, degree: 180 };
  let ball2 = ball; // こちらもCopyトレイトを実装している型なのでok
  assert_eq!(ball.x, ball2.x);
  assert_eq!(ball.y, ball2.y);
  assert_eq!(ball.degree, ball2.degree);

  // 関数に束縛を渡した場合も, 所有権を渡さず完全なコピーを渡す
  let x = 5;
  let x2 = double(x);
  assert_eq!(x * 2, x2);

  let a = true;
  let b = change_truth(a);
  assert!(a);
  assert!(!b);

}

