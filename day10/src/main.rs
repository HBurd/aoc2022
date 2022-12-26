//struct CPU {
//    cycle: i32,
//    X: i32,
//}
//
//impl CPU {
//    fn ex(&mut self, instr: &str) -> i32 {
//        let mut words = instr.split(' ');
//        match (words.next(), words.next()) {
//            (Some("addx"), Some(x)) => {
//                if let Ok(x_int) = x.parse::<i32>() {
//                    self.X += x_int;
//                    self.cycle += 2;
//                }
//                else {
//                    panic!("Not an int!");
//                }
//            },
//            (Some("noop"), None) => { self.cycle += 1 },
//            _ => { panic!("Unknown instruction!") },
//        }
//    }
//}

enum Instr {
    AddX(i32),
    NoOp,
}

fn main() {
    let input_str = include_str!("../test.txt");
    let mut lines = input_str.lines();

    let cycles_of_interest = [20, 60, 100, 140, 180, 220];

    let mut cycle = 0;
    let mut X = 1;
    let mut instr: Option<Instr> = None;
    let mut sum = 0;

    loop {

        if let Some(Instr::AddX(x)) = instr {
            X += x;
            instr = None;
        }
        else if let Some(_) = instr {
            panic!();
        }
        else if let Some(line) = lines.next() {
            let mut words = line.split(' ');
            match words.next() {
                Some("addx") => {
                    if let Some(x) = words.next().and_then(|x| x.parse::<i32>().ok()) {
                        println!("{}", x);
                        instr = Some(Instr::AddX(x));
                    }
                    else {
                        panic!();
                    }
                },
                Some("noop") => {},
                _ => { panic!() },
            }
        }
        else {
            break;
        }

        println!("tick");

        
        cycle += 1;

        if cycles_of_interest.contains(&cycle) {
            println!("{}, {}, {}", X, cycle, X * cycle);
            sum += X * cycle;
        }
    }
    println!("Sum: {}", sum);
}
