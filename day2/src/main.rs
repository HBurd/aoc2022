fn main() {
    let input = include_str!("../input.txt");
    let mut chars = input.chars();
    let mut score = 0;
    let mut score2 = 0;

    loop {

        let theirplay = match chars.next() {
            Some('A') => 0,
            Some('B') => 1,
            Some('C') => 2,
            _ => -1,
        };

        if theirplay == -1 {
            break;
        }

        // Skip over space
        chars.next();

        let myplay = match chars.next() {
            Some('X') => 0,
            Some('Y') => 1,
            Some('Z') => 2,
            _ => -1,
        };

        if myplay == -1 {
            break;
        }

        // Part 1

        let diff = (myplay - theirplay + 3) % 3;

        score += match diff {
            0 => 3, // tie
            1 => 6, // win
            _ => 0, // loss or impossible
        };

        score += myplay + 1;

        // Part 2

        score2 += myplay * 3; // win lose tie

        let myrealplay = match myplay {
            0 => theirplay + 2,
            1 => theirplay,
            2 => theirplay + 1,
            _ => -1,
        } % 3;
        
        score2 += myrealplay + 1;

        chars.next();
    }

    println!("Score1: {}", score);
    println!("Score2: {}", score2);
}
