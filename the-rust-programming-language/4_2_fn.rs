
// 返り値のなし、引数なしの関数
fn say_foo() {
  println!("foooooooooooooooo!!!!!!!!!!!!!!!");
}

// 引数有りの関数
// 引数はlet宣言とほぼ同じ働きをするが, 型宣言必要
fn say_number(x: i32) {
  // x = 10; // immutableなのでerror
  println!("x is {}", x);
}

fn sum(a: i32, b: i32) {
  println!("sum : {}", a + b);
}

// 戻り値ありの関数
// fn name(arg1: arg1_type, ...) -> 返却型名
fn add_one(n: i32) -> i32 {
  n + 1 // 関数の最後に評価した値が返却値になるが(like perl, scala, ruby), セミコロンをつけるとエラー発生(式でないため)
}

// これダメ, 文で終わっているので型が違う
/* fn add__(n: i32) -> i32 {
  n + 1;
} */
// 返却型書かなければok

// 式と文について
// 文は;で終わる, 値を返さない(厳密には())
// 式は;で終わらない, だが値を返す
// 厳密に言うと、無数の式を区切るために;があり、

fn add_two(n: i32) -> i32 {
  return n + 2; // ちなみに、returnキーワードを使えば文を式として返却値を帰すことができる
  println!("prprpprp");
  // 関数の最後にreturn書いてもいいけど,rust的でない
}

// 発散する関数 (=決して値を返さない関数,厳密な意味でも返り値がない)
// panic! は実行中の現在のスレッドを与えられたメッセージとともにクラッシュさせます。 
// 使いドコロがわからん
fn diverges() -> ! {
  panic!("この関数は発散しまっっっす");
}


fn main() {

  // 関数呼び出し
  say_foo();
  // cと違い先に宣言しなくても使え、errorもwarningもない
  say_foo_2();

  say_number(110);
  sum(25, 34);
  println!("add_one : {}", add_one(2));

  // 幾つかの言語では変数を代入した際に値を返すが、rustは返さない(厳密には())
  // また、変数束縛キーワードletは文の先頭にしかおけない, それ以外はerror
  // let x = (let y = 5); // error
  let mut y = 8;
  let x = (y = 6); // これは動くが、変数代入の際に返す値は() xには()が入ることになる

  println!("add_two: {}", add_two(20));

  // diverges();
  // 発散する関数は任意の型としても動くが意味がわからん
  // let x: i32 = diverges();

  // 関数ポインタ
  // f は i32 を引数として受け取り、 i32 を返す関数を指示する変数束縛
  // 関数add_oneのポインタを束縛
  let f: fn(i32) -> i32 = add_one;
  let f = add_one;
  // 関数に別名つける感覚と同じ? ()で呼び出せる
  println!("f : {}", f(2));

}

fn say_foo_2() {
  println!("fooooooooooooooo!!!!!!!!!!!!!111111111111111!!!!1!!!");
}

