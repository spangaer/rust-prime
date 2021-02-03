use std::time::SystemTime;
use std::vec;

fn main() {
    let batch = 1024;

    let mut primes: vec::Vec<u64> = vec![2];
    let mut squares: vec::Vec<u64> = vec![4];

    let mut top: u64 = 2;
    // let mut top_quad: u64 = 4;
    let mut last_end = 3;

    let now = SystemTime::now();

    while match now.elapsed() {
        Ok(e) => e.as_secs() < 60,
        Err(e) => {
            eprint!("{}", e);
            false
        }
    } {
        let chunk = if top >= batch { batch } else { top };
        let next_end = last_end + chunk;

        let (new_primes, new_squares) = prime_from(&primes, &squares, last_end, next_end);
        primes.extend(new_primes);
        squares.extend(new_squares);
        last_end = next_end;
        top = last_end - 1;
        // top_quad = top * top;
    }

    println!(
        "{} nth prime: {}",
        primes.len(),
        primes.last().unwrap_or(&0)
    )
}

fn prime_from(
    primes: &vec::Vec<u64>,
    squares: &vec::Vec<u64>,
    start: u64,
    end: u64,
) -> (vec::Vec<u64>, vec::Vec<u64>) {
    let numbers = start..end;

    let new_primes: vec::Vec<u64> = numbers
        .filter(|n| {
            !primes
                .iter()
                .zip(squares.iter())
                .take_while(|(_, &s)| s <= *n)
                .any(|(p, _)| n % p == 0)
        })
        .collect();

    let new_squares = new_primes.iter().map(|x| x * x).collect();

    (new_primes, new_squares)
}
