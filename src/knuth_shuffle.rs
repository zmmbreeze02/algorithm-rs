
pub fn knuth_shuffle<T>(input: &mut [T]) {
    for i in (0..input.len()).rev() {
        // Swap elements
        input.swap(i, rand::random::<usize>() % (i + 1));
    }
}



#[cfg(test)]
mod tests {
    use crate::knuth_shuffle::knuth_shuffle;

    #[test]
    fn test_knuth_shuffle() {
        let mut input = [1, 2, 3, 4, 5];
        knuth_shuffle(&mut input);
        println!("{:?}", input);
    }
}
