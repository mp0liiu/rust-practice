// 参照と借用

fn main() {

  /* 参照渡し */
  let (v1, v2) = (vec![0, 2, 3], vec![1, 5, -3]);
  assert_eq!(borrow_ex_1(&v1, &v2), 5);

  /* muttableな参照 */
  let mut x = 3;
  {
    // 束縛xのmuttableな参照を取得
    // 借用のスコープは、束縛のスコープより短くなければいけない
    let y = &mut x;
    // '*'で参照元の内容にアクセス
    *y += 1;
  }
  assert_eq!(x, 4);

  /* 借用についてのルール */
  // 1. リソースに対する1つ以上のimmutableな参照 &T
  // 2. ただ1つのミュータブルな参照 &mut T
  // 3. 借用は全て所有者のスコープより長生きしてはいけない
  // -> 借用のスコープは、束縛のスコープより短くなければいけない
  // これらはデータ競合を防ぐため
  // ちなみにimmutableとmuttableの参照は同時に取得できない
  // -> データが更新された場合immutableな参照の方は実質muttableになるから

  // 練習 3人の生徒の点数の合計をもとめる関数 借用使用番
  let mut students = vec![
    Student { name: "高山春香".to_string(), age: 16, score: 90 },
    Student { name: "園田優".to_string(), age: 16, score: 55 },
    Student { name: "野田コトネ".to_string(), age: 16, score: 73 },
    Student { name: "南しずく".to_string(), age: 16, score: 82 },
  ];
  println!("{}", student_score_sum(&mut students));

}

fn borrow_ex_1(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
  v1[0] + v2[1]
}

fn student_score_sum(students: &mut Vec<Student>) -> i32 {
  for s in students.into_iter() {
    print!("{} ", s.name);
  }
  print!("\n");
  students.sort_by(|a, b| { a.score.cmp(&b.score) });
  println!("top score : {}", students[2].score);
  students.sort_by(|a, b| { a.age.cmp(&b.age) });
  println!("top age : {}", students[2].age);
  let sum = students.iter().fold(0, |sum, student| {
    sum + student.score as i32
  });
  sum
}

struct Student {
  name: String,
  age: i8,
  score: i8,
}
