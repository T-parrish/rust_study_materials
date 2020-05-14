extern crate rand;
// use rand::Rng;
use rand::distributions::{Distribution, Uniform};

pub fn gen_random_vector(start: i32, stop: i32, count: u32) -> Vec<i32> {
    let step = Uniform::new(start, stop);
    let mut rng = rand::thread_rng();
    let choices: Vec<_> = step.sample_iter(&mut rng).take(count as usize).collect();

    choices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_gen() {
        let vec1 = gen_random_vector(0, 10, 5);
        println!("vec1: {:?}", vec1);

        assert!(vec1.len() == 5);
        
        for el in vec1 {
            assert!(el < 10);
        };
        
    }

}