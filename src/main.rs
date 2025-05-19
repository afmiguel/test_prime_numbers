use primal::is_prime;
use std::time::Instant;

const MAX_VALUE: u64 = 50_000_000;

#[allow(dead_code)]
const NUM_THREADS: usize = 10;

fn main() {
    println!("Sequential Execution");
    let start = Instant::now();
    let mut count_primes = 0;
    for n in 0..MAX_VALUE{
        if is_prime(n) {
            count_primes += 1;
        }
    }
    let duration = start.elapsed();
    println!("There are {} primes", count_primes);
    println!("Time taken: {:?}", duration);
}
