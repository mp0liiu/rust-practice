fn main() {

  // 変数束縛にはスコープ内でのみ有効
  let x = 17;
  {
    let y = 3;
    println!("x => {}, y => {}", x, y);
  }

  // yの宣言されているスコープの外から参照しようとしているためエラー
  // println!("x => {}, y => {}", x, y);
  println!("x => {}", x);

  // 変数を再束縛することで変数を上書きすることができる(シャドーイング)
  // 下ではスコープを新たに作り、そのスコープ内だけで有効なシャドーイングをおこなっている
  // スコープ内で宣言するまでスコープ外の変数を参照する
  println!("== shadowing ==");
  {
    println!("x => {}", x); // これerrrorにならん, 17とでる
    let x = 23;
    println!("x => {}", x); // 23
  }

  // 同じスコープ内でもシャドーイング可能
  // 他言語なら再宣言の警告、エラーが出るがrustでは違う,問題のない記述

  // シャドーイングとミュータブルな束縛の違い
  // シャドーイングは変数の型も替えられる, まあ当たり前

  let y = 4;
  println!("y => {}", y);
  let y = "shadowing"; // 型が違うがシャドーイングなので問題ない
  println!("y => {}", y);

  println!("======");
  let mut x: i32 = 100;
  println!("x => {}", x);
  x = 7;
  println!("x => {}", x);
  // x = "string"; // error

  // ちなみに、immutableな束縛にシャドーイングでもう一度束縛し直すことで変数をimmutableに変えることもできる
  let x = x;
  println!("x => {}", x);
  //x = 10; // error

  // 感想, スコープを限定したシャドーイングはいいとおもう(Perlと似た感覚)
  // あまりないことだと思うが同じスコープでのシャドーイングは実質mutableな変数を使うのとかわらなくて
  // 危険なのでは

}
