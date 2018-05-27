use std::collections::{ HashMap, VecDeque };

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

struct CharsInfo {
    chars: VecDeque<char>,
    east_num: i32,
    west_num: i32,
}

fn char_nums(chars: &VecDeque<char>) -> HashMap<char, i32> {
    let mut counter = HashMap::new();
    for c in chars {
        let mut count = counter.entry(*c).or_insert(0);
        *count += 1;
    }
    counter
}

fn main() {
    let n: i32       = read();
    let vectored_str = read::<String>().chars().collect::<Vec<char>>();
    let mut right = CharsInfo {
        chars: VecDeque::new(),
        east_num: 0,
        west_num: 0,
    };
    let mut left = {
        let chars: VecDeque<char> = (1 .. n).map(|i| vectored_str[i as usize]).collect();
        let counter = char_nums(&chars);
        CharsInfo {
            chars: chars,
            east_num: *counter.get(&'E').unwrap_or(&0),
            west_num: *counter.get(&'W').unwrap_or(&0),
        }
    };

    let change_nums: Vec<i32> = (0 .. n).map(|leader_index| {
        let right_change_num = right.west_num;
        let left_change_num  = left.east_num;

        let c = vectored_str[leader_index as usize];
        right.chars.push_back(c);
        if c == 'E' { right.east_num += 1 }
        if c == 'W' { right.west_num += 1 }

        let remove_char = left.chars.pop_front().unwrap_or('\0');
        if remove_char == 'E'  { left.east_num -= 1 }
        if remove_char == 'W'  { left.west_num -= 1 }

        right_change_num + left_change_num
    }).collect();
    println!("{}", change_nums.iter().min().unwrap());
}

