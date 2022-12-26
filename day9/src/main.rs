use std::ops;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(i32, i32);

impl TryFrom<&str> for Coord {
    type Error = ();

    fn try_from(value: &str) -> Result<Coord, Self::Error> {
        let mut words = value.split(' ');

        let dir_opt = words.next();
        let amt_opt = words.next().and_then(|x| x.parse::<i32>().ok());

        match (dir_opt, amt_opt) {
            (Some("U"), Some(amt)) => Ok(Coord(0, amt)),
            (Some("D"), Some(amt)) => Ok(Coord(0, -amt)),
            (Some("L"), Some(amt)) => Ok(Coord(-amt, 0)),
            (Some("R"), Some(amt)) => Ok(Coord(amt, 0)),
            _ => Err(()),
        }
    }
}

impl ops::Add<Coord> for Coord {
    type Output = Self;

    fn add(self, rhs: Coord) -> Coord {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub<Coord> for Coord {
    type Output = Self;

    fn sub(self, rhs: Coord) -> Coord {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Coord {
    fn follow(&mut self, head: Coord) {
        let delta = head - *self;
        if delta.0.abs() > 1 || delta.1.abs() > 1 {
            *self = *self + delta.normalized();
        }
    }

    fn normalized(&self) -> Coord {
        let mut result = Coord(0, 0);
        
        if self.0 != 0 {
            result.0 = self.0 / self.0.abs();
        }
        if self.1 != 0 {
            result.1 = self.1 / self.1.abs();
        }

        return result;
    }
}

fn main() {
    let input_str = include_str!("../input.txt");

    //let mut head = Coord(0, 0);
    //let mut tail = Coord(0, 0);
    let mut rope = [Coord(0, 0); 10];

    let mut visited = HashSet::new();
    visited.insert(rope[0]);

    for line in input_str.lines() {
        // Why did I make this so complicated
        // just hacking things together now
        let next_mv = Coord::try_from(line).unwrap();
        let mut delta = Coord(0, 0);
        while delta != next_mv {
            rope[0] = rope[0] + next_mv.normalized();
            delta = delta + next_mv.normalized();
            for i in 1..rope.len() {
                rope[i].follow(rope[i - 1]);
            }
            visited.insert(*rope.last().unwrap());
        }
    }

    println!("{:?}, {:?}, {}", rope[0], rope.last().unwrap(), visited.len());
}
