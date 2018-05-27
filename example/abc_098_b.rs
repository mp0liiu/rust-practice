use std::collections::HashSet;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn chars(s: &str) -> HashSet<char> {
    let mut chars = HashSet::new();
    for c in s.chars() {
        chars.insert(c);
    }
    chars
}

fn same_chars(right_chars: HashSet<char>, left_chars: HashSet<char>) -> Vec<char> {
    let chars: Vec<char> = right_chars.iter().filter(|c| {
        right_chars.get(c).is_some() && left_chars.get(c).is_some()
    }).map(|c| *c).collect();
    chars
}

fn main() {
    let n: i8     = read();
    let s: String = read();
    let char_types_counts: Vec<i8> = (1 .. n).map(|separate_index| {
        let (right, left) = s.split_at(separate_index as usize);
        let same_chars    = same_chars( chars(right), chars(left) );
        same_chars.len() as i8
    }).collect();
    println!("{}", char_types_counts.iter().max().unwrap_or(&0));
}
