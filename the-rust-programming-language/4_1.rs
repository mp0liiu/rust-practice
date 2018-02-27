fn main() {

  // 変数に束縛 rustでは1つの値に対して1つの名前が対応する = 別名はつけられない
  let x = 5;

  // パターンを使って一度に代入
  let (a, b) = (1, 2);

  println!("x => {}, a => {}, b => {}", x, a, b);

  // 型推論がある 普段は省略可
  let num: i32 = 8;

  // 数値型
  // 符号付き型はi, 符号なし型はu + bit数
  let num1: i8 = -12;
  let num2: u8 = 25;

  println!("num => {}, num1 => {}, num2 => {}", num, num1, num2);

  // 変数はdefaultでimmutable
  let z = 1;
  // z = 23; -> error

  // mutableな変数を使いたいなら mut を使う
  let mut m = 20;
  // 変数を書き換える前に書き換え前の変数を使わないままでいると、警告が出る
  println!("m => {}", m);
  m = 10;

  // rustでは必ず初期化する必要がある
  // このままun_init変数を使うとerror
  // つかわなければwarning
  let un_init;

  // 後からでもいいので、使用前に初期化すればerrorにならない
  un_init = "prprpr";

  println!("z => {}, m => {}, un_init => {}", z, m, un_init);

}
