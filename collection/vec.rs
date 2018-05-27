use std::vec::Vec;

fn main() {
    let mut v = (0 .. 10).collect::<Vec<i8>>().to_vec();
      // ちなみに Perlみたいに vec![0 .. 9] みたいなことはできない
      // Vecの中にrange構造体を入れる形になる
    // コレクション内の要素の所有権は「コレクションの要素」が持っているため、
    // let num = v[0];
    // みたいなことをすると、所有権がコレクションから離れてしまい、(所有権の管理がとても複雑になるため?)
    // コレクション内の要素を直接取ってくるみたいなことはできなくなっている
    // 代わりに、コレクションのメソッドを使ってコレクション内の要素の参照を取得する
    // コレクション内の要素を変更したい場合は、muttableな参照を取得する
    {
      let num = v.get(0).unwrap(); // そももそも、 v[0] = v.get(0).unwrap() の糖衣構文なのでは？
      assert_eq!(v[0], *num);
      assert_eq!(*num, 0);
    }
    if let Some(num) = v.get_mut(0) {
        *num = 122;
    }
    assert_eq!(v[0], 122);
    if let Some(num) = v.get_mut(9) {
        *num = 99;
    }
    for num in v.iter() {
        println!("{}", num);
    }
}
