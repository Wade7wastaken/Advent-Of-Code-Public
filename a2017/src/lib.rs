use lib::itertools::Itertools;

#[must_use]
pub fn vec_reverse(mut list: Vec<u8>, i: usize, len: usize) -> Vec<u8> {
    let size = list.len();
    for x in 0..(len / 2) {
        list.swap((i + x) % size, (i + len - 1 - x) % size);
    }
    list
}


pub fn knot_hash(input: &str) -> Vec<u8> {
    std::iter::repeat_n((), 64)
        .flat_map(|()| input.bytes().chain([17, 31, 73, 47, 23]))
        .map(usize::from)
        .enumerate()
        .fold(((0..=255).collect_vec(), 0), |(list, i), (skip, length)| {
            (vec_reverse(list, i, length), i + length + skip)
        })
        .0
        .chunks_exact(16)
        .map(|chunk| chunk.iter().copied().reduce(|x, acc| acc ^ x).unwrap())
        .collect()
}