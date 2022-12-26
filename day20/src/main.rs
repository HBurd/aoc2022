fn modulo(a: isize, b: usize) -> usize
{
    ((a % b as isize + b as isize) % b as isize) as usize
}

fn main()
{
    let input = include_str!("../input.txt");
    let sequence = input.lines().map(|line| line.parse::<isize>().unwrap() * 811589153);

    let mut index_sequence: Vec<(usize, isize)> = Vec::new();

    for el in sequence.enumerate()
    {
        index_sequence.push(el)
    }

    let len = index_sequence.len();

    println!("{:?}", index_sequence);

    for _ in 0..10
    {
        for i in 0..index_sequence.len()
        {
            for j in 0..index_sequence.len()
            {
                if i == index_sequence[j].0
                {
                    let el = index_sequence.remove(j);
                    let index = modulo(j as isize + el.1, len - 1);

                    index_sequence.insert(index, el);
                    //println!("{:?}", index_sequence);

                    break;
                }
            }
        }
    }

    let mut start = 0;

    for i in 0..index_sequence.len()
    {
        if index_sequence[i].1 == 0 {
            start = i;
        }
    }

    let coord1 = index_sequence[(start + 1000) % len].1;
    let coord2 = index_sequence[(start + 2000) % len].1;
    let coord3 = index_sequence[(start + 3000) % len].1;
    println!("{} + {} + {} = {}", coord1, coord2, coord3, coord1 + coord2 + coord3);
}
