// ライフタイム = 変数の生存期間 = スコープの範囲みたいなかんじ

// 借用のライフタイムを省略した関数
fn foo(x: &i32) {
  println!("foo: {}", x);
}

// fooのライフタイムを省略せず明示した関数
// 'a = ライフタイムa
// 関数foo2にはライフタイムaがあり、仮引数xはライフタイムaを持つi32型の参照
// ライフタイムを直接定義できるのは仮引数のみ？
fn foo2<'a, 'b>(x: &'a i32) {
  println!("foo2: {}", x);
  
  // ライフタイムを明示的に宣言した変数束縛
  let _num_ref: &'a i32 = x;

  // this is error, 定義されていないライフタイムは自動的にstaticになる
  // let _num: &'b i32 = x;

  // つまりこれは行ける(文字列はstaticライフタイムの参照なので)
  let _string_1: &'b str = "HELLO, WORLD?";
  let _string_2: &'static str = "HELLO, WORLD?";
  let _string_3 = "HELLO, WORLD?";
}

// muttable
fn foo3<'a>(x: &'a mut i32) {
  println!("foo3: {}", x);
}

// 2つの参照引数がある場合
fn xor<'a, 'b>(a: &'a i32, b: &'b i32) {
  println!("two arguments xor: {}", a ^ b);
}

// 2つの参照引数がある場合(同一のライフタイム)
fn and<'a>(a: &'a i32, b: &'a i32) {
  println!("two arguments and: {}", a & b);
}

fn bound<'a>(a: &'a i32) -> &'a i32 {
  println!("参照をそのまま返す {}", a);
  a
}

struct RefPair<'a> {
  a: &'a i32,
  b: &'a i32,
}

fn main() {

  let x = 20;
  foo(&x);
  foo2(&x);

  // let y = 10;
  // let y_ref = &mut 10;
  let y = &mut 10;
  foo3(y);

  xor(&3, &5);
  and(&2, &3);

  let num1 = 4;
  {
    let num2 = 10;
    xor(&num1, &num2);
    and(&num1, &num2);
  }

  let _x_ref: &i32 = bound(&x);

  let a = 0;
  {
    let pair = RefPair { a: &a, b: &1 };
    println!("RefPair [ a: {}, b: {} ]", pair.a, pair.b);
  }
  println!("a: {}", a);

}

