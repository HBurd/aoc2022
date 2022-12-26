use std::collections::VecDeque;

enum Op {
    Add(u64),
    Mult(u64),
    Sqr
}

impl Op {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Op::Add(y) => x + y,
            Op::Mult(y) => x * y,
            Op::Sqr => x * x,
        }
    }
}

struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test: u64,
    iftrue: usize,
    iffalse: usize,
    inspections: u64,
}

impl Monkey {
    fn inspect(&mut self) -> Option<(u64, usize)> {
        if let Some(mut x) = self.items.pop_front() {
            self.inspections += 1;
            x = self.op.apply(x);
            x = x % 9699690;
            //x /= 3;
            if x % self.test == 0 {
                return Some((x, self.iftrue));
            }
            else {
                return Some((x, self.iffalse));
            }
        }
        return None;
    }
}

fn main() {
    let mut monkeys = [
        Monkey {
            items: VecDeque::from([98, 70, 75, 80, 84, 89, 55, 98]),
            op: Op::Mult(2),
            test: 11,
            iftrue: 1,
            iffalse: 4,
            inspections: 0 },
        Monkey {
            items: VecDeque::from([59]),
            op: Op::Sqr,
            test: 19,
            iftrue: 7,
            iffalse: 3,
            inspections: 0 },
        Monkey {
            items: VecDeque::from([77, 95, 54, 65, 89]),
            op: Op::Add(6),
            test: 7,
            iftrue: 0,
            iffalse: 5,
            inspections: 0 },
        Monkey {
            items: VecDeque::from([71, 64, 75]),
            op: Op::Add(2),
            test: 17,
            iftrue: 6,
            iffalse: 2,
            inspections: 0 },
        Monkey {
            items: VecDeque::from([74, 55, 87, 98]),
            op: Op::Mult(11),
            test: 3,
            iftrue: 1,
            iffalse: 7,
            inspections: 0 },
        Monkey {
            items: VecDeque::from([90, 98, 85, 52, 91, 60]),
            op: Op::Add(7),
            test: 5,
            iftrue: 0,
            iffalse: 4,
            inspections: 0 },
        Monkey {
            items: VecDeque::from([99, 51]),
            op: Op::Add(1),
            test: 13,
            iftrue: 5,
            iffalse: 2,
            inspections: 0 },
        Monkey {
            items: VecDeque::from([98, 94, 59, 76, 51, 65, 75]),
            op: Op::Add(5),
            test: 2,
            iftrue: 3,
            iffalse: 6,
            inspections: 0 },
    ];

    for _ in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            while let Some((item, dest)) = monkeys[monkey_index].inspect() {
                monkeys[dest].items.push_back(item);
            }
        }
    }

    let mut max1 = 0;
    let mut max2 = 0;

    for monkey in monkeys.iter() {
        if monkey.inspections >= max1 {
            max2 = max1;
            max1 = monkey.inspections;
        }
    }

    println!("{}, {}, {}", max1, max2, max1 * max2);
}
