pub mod statistics {
    use rand::Rng;

    pub fn generate_random_numbers(i: u8, k: u8, n: u8) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut numbers: Vec<u8> = (i..=k).collect(); 
        let mut unique_indices = Vec::with_capacity(n as usize);

        for _ in 0..n {
            let range = (k - i + 1) as usize - unique_indices.len(); 
            let index = rng.gen_range(0..range); 
            unique_indices.push(numbers[index] as usize); 
            numbers.swap_remove(index); 
        }

        unique_indices
    }
}
