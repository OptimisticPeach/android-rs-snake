extern crate rand;

pub fn rand_range(start: usize, end: usize) -> u32 {
    let mut num: usize = rand::random();
    num %= end - start;
    num += start;
    num as u32
}
