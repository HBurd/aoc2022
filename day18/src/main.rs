type Coord = (u32, u32, u32);

fn read_point(line: &str) -> (u32, u32, u32) {
    let mut words = line.split(',');
    // Add 5 to hack around underflow :)
    (words.next().unwrap().parse::<u32>().unwrap() + 5,
     words.next().unwrap().parse::<u32>().unwrap() + 5,
     words.next().unwrap().parse::<u32>().unwrap() + 5)
}

struct BitSet {
    bits: [u32; 1024],
}

impl BitSet {
    fn new() -> BitSet{
        BitSet{ bits: [0; 1024] }
    }

    fn set(&mut self, coord: &Coord) {
        let index = Self::get_index(coord);
        self.bits[index / 32] |= 1 << (index % 32);
    }

    fn get(&self, coord: &Coord) -> bool{
        let index = Self::get_index(coord);
        self.bits[index / 32] & (1 << (index % 32)) != 0
    }

    fn get_index((x, y, z): &Coord) -> usize {
        let stride = 32;
        return (x + y * stride + z * stride * stride) as usize;
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut pixels = BitSet::new();
    let mut area: u32 = 0;

    for line in input.lines() {
        let (x, y, z) = read_point(line);

        let adjacents = [(x + 1, y, z), (x - 1, y, z),
                         (x, y + 1, z), (x, y - 1, z),
                         (x, y, z + 1), (x, y, z - 1)];

        if !pixels.get(&(x, y, z)) {
            area += 6;
            for adj in adjacents.iter() {
                if pixels.get(adj) {
                    area -= 2;
                }
            }
        }

        pixels.set(&(x, y, z));
    }
    
    println!("{}", area);

    let mut area2 = 0;

    let mut visited = BitSet::new();
    visited.set(&(1, 1, 1));

    let mut repeat = true;
    while repeat {
        repeat = false;
        let mut seen = 0;
        for z in 1..31 {
            for y in 1..31 {
                for x in 1..31 {

                    if !pixels.get(&(x, y, z)) && !visited.get(&(x, y, z)) {
                        seen += 1;
                        let adjacents = [(x + 1, y, z), (x - 1, y, z),
                                         (x, y + 1, z), (x, y - 1, z),
                                         (x, y, z + 1), (x, y, z - 1)];

                        if x == 7 && y == 7 && z == 7 {
                            println!("HERE!");
                        }

                        let mut adjacent_area = 0;
                        let mut exterior = false;
                        for adj in adjacents.iter() {
                            exterior |= visited.get(adj);
                            if pixels.get(adj) {
                                adjacent_area += 1;
                            }
                        }

                        if exterior {
                            area2 += adjacent_area;
                            visited.set(&(x, y, z));
                            repeat |= adjacent_area > 0;
                        }
                    }
                }
            }
        }
        println!("seen: {}", seen);
    }

    println!("{}", area2);
}
