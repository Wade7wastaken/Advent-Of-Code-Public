fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

const fn run(mut a: u32) {
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;

    d = a; // cpy a d
    c = 14; // cpy 14 c
    loop {
        b = 182; // cpy 182 b
        loop {
            d += 1; // inc d
            b -= 1; // dec b
            if b == 0 {
                break;
            }
        } // jnz b -2
        c -= 1; // dec c
        if c == 0 {
            break;
        }
    } // jnz c -5
    a = d; // cpy d a
    // jnz 0 0
    b = a; // cpy a b
    a = 0; // cpy 0 a
    c = 2; // cpy 2 c
    loop {
        if b != 0 { // jnz b 2
            // jnz 1 6
        }
        b -= 1; // dec b
        c -= 1; // dec c
        if c == 0 {
            break;
        }
    } // jnz c -4
    a += 1; // inc a
    // jnz 1 -7
    // cpy 2 b
    // jnz c 2
    // jnz 1 4
    // dec b
    // dec c
    // jnz 1 -4
    // jnz 0 0
    // out b
    // jnz a -19
    // jnz 1 -21
}

fn part1(input: &str) -> u32 {
    todo!();
}

fn part2(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        // assert_eq!(part1(input), todo!());
        // assert_eq!(part2(input), todo!());
    }
}
