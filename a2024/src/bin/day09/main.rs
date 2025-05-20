use lib::{itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Block {
    Free,
    File(u64),
}

fn part1(input: &str) -> u64 {
    let mut fs = vec![];
    let mut free = false;
    let mut id = 0;
    for d in input.chars().map(|c| c.to_digit(10).unwrap() as usize) {
        if free {
            fs.append(&mut vec![None; d]);
        } else {
            fs.append(&mut vec![Some(id); d]);
            id += 1;
        }
        free = !free;
    }
    let mut sum = 0;

    let mut it = fs.into_iter();

    let mut i = 0;

    while let Some(block) = it.next() {
        let id = match block {
            None => {
                let mut from_back = None;
                while from_back.is_none() {
                    from_back = it.next_back().unwrap_or(Some(0));
                }
                from_back.unwrap()
            }
            Some(id) => id,
        };
        sum += i * id;
        i += 1;
    }

    sum
}

#[derive(Debug)]
enum Chunk {
    Free(usize),
    File(usize, u64),
}

fn part2(input: &str) -> u64 {
    let mut chunks = vec![];
    let mut free = false;
    let mut id = 0;
    for d in input.chars().map(|c| c.to_digit(10).unwrap() as usize) {
        if free {
            chunks.push(Chunk::Free(d));
        } else {
            chunks.push(Chunk::File(d, id));
            id += 1;
        }
        free = !free;
    }

    let mut i = chunks.len() - 1;

    while i > 0 {
        match chunks[i] {
            Chunk::Free(_) => {}
            Chunk::File(size, id) => {
                if let Some(next_free_i) = chunks.iter().position(|c| match c {
                    Chunk::Free(free_size) => *free_size >= size,
                    Chunk::File(_, _) => false,
                }) {
                    if next_free_i < i {
                        let chunk_removed = chunks.remove(i);
                        let chunk_removed_len = match chunk_removed {
                            Chunk::Free(_) => unreachable!(),
                            Chunk::File(len, _) => len,
                        };
                        chunks.insert(i, Chunk::Free(chunk_removed_len));
                        chunks.insert(next_free_i, chunk_removed);

                        match chunks[next_free_i + 1] {
                            Chunk::File(_, _) => unreachable!(),
                            Chunk::Free(ref mut size) => *size -= chunk_removed_len,
                        }
                        i += 1;
                    }
                }
            }
        }
        i -= 1;
    }

    let mut pos = 0;
    let mut sum = 0;

    for chunk in chunks {
        match chunk {
            Chunk::Free(size) => {
                pos += size as u64;
            }
            Chunk::File(size, id) => {
                for _ in 0..size {
                    sum += id * pos;
                    pos += 1;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 6349606724455);
        assert_eq!(part2(input), 6376648986651);
    }
}
