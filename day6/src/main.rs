fn check_distinct4(chars: &[char; 4]) -> bool {
    chars[0] != chars[1]
    && chars[0] != chars[2]
    && chars[0] != chars[3]
    && chars[1] != chars[2]
    && chars[1] != chars[3]
    && chars[2] != chars[3]
}

fn check_distinct14(chars: &[char; 14]) -> bool {
    for i in 0..chars.len() {
        for j in (i + 1)..chars.len() {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }

    return true;
}

fn main() {
    let input_str = include_str!("../input.txt");
    let mut input_chars = input_str.chars();

    let mut chars = [' '; 14];
    let mut i: usize = 0;

    loop {
        chars[i % 14] = input_chars.next().unwrap();
        //if i > 3 && check_distinct4(&chars) {
        if i > 14 && check_distinct14(&chars) {
            break;
        }

        i += 1;
    }

    println!("{}", i + 1);
}
