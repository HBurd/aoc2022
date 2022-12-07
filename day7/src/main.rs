type Lines<'a> = std::iter::Peekable<std::str::Lines<'a>>;

fn size_from_ls<'a>(input_lines: &mut Lines<'a>) -> u32 {
    let mut size: u32 = 0;
    loop {
        if let Some(line) = input_lines.peek() {

            let mut line_words = line.split(" ");
            if let Some(first_word) = line_words.next() {
                if first_word == "$" {
                    return size;
                }
                
                if let Ok(x) = first_word.parse::<u32>() {
                    size += x;
                }
            }
            // Consume line
            input_lines.next();
        }
        else {
            // EOF
            return size;
        }
    }
}

fn get_directory_size<'a>(input_lines: &mut Lines<'a>) -> (u32, u32, u32) {
    let mut size: u32 = 0;
    let mut minsize: u32 = 0;
    let mut mincandidate: u32 = 100000000;

    loop {
        if let Some(line) = input_lines.next() {
            let mut command_words = line.split(' ');

            if command_words.next() != Some("$") {
                panic!();
            }

            match command_words.next() {
                Some("ls") => { size += size_from_ls(input_lines); },
                Some("cd") => {
                    if command_words.next() == Some("..") {
                        break;
                    }
                    else {
                        let (dsize, msize, mcandidate) = get_directory_size(input_lines);
                        size += dsize;
                        minsize += msize;
                        mincandidate = std::cmp::min(mincandidate, mcandidate);
                    }
                },
                Some(other) => { panic!("Got {}!", other); }
                None => { panic!("Empty line!"); }
            }
        }
        else {
            break;
        }
    }

    if size >= 3313415 {
        mincandidate = std::cmp::min(mincandidate, size);
    }

    return (size, if size <= 100000 { size + minsize } else { minsize }, mincandidate);
}

fn main() {
    let input_str = include_str!("../input.txt");
    let mut input_lines = input_str.lines().peekable();

    // Discard first line `$ cd /`
    input_lines.next();

    let (size, minsize, mincandidate) = get_directory_size(&mut input_lines);
    println!("Total size: {}", size);
    println!("Sum of small: {}", minsize);

    let remaining = 70000000 - size;
    println!("Remaining: {}", remaining);
    println!("Delete: {}", 30000000 - remaining);
    println!("Min candidate: {}", mincandidate);
}
