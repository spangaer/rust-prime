use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;
use std::vec;

struct Batch {
    primes: Arc<vec::Vec<u64>>,
    squares: Arc<vec::Vec<u64>>,
    start: u64,
    end: u64,
}

impl Clone for Batch {
    fn clone(&self) -> Batch {
        Batch {
            primes: self.primes.clone(),
            squares: self.squares.clone(),
            start: self.start,
            end: self.end,
        }
    }
}

struct Report {
    primes: vec::Vec<u64>,
    squares: vec::Vec<u64>,
    start: u64,
    end: u64,
    index: usize,
}

fn main() {
    let batch = 4194304; //1048576; //65536; //262144; //1024;
    let threads = 12;

    let mut working = vec![false; threads];
    let mut channels = vec::Vec::new();
    let (main_tx, main_rx) = mpsc::channel::<Report>();

    for i in 0..working.len() {
        let (tx, rx) = mpsc::channel::<Batch>();
        channels.push(tx);
        let main_tx = main_tx.clone();
        thread::spawn(move || loop {
            match rx.recv() {
                Ok(b) => {
                    let (new_primes, new_squares) =
                        prime_from(&b.primes, &b.squares, b.start, b.end);
                    let report = Report {
                        primes: new_primes,
                        squares: new_squares,
                        start: b.start,
                        end: b.end,
                        index: i,
                    };

                    main_tx.send(report).unwrap()
                }
                Err(e) => {
                    eprint!("{}", e);
                    break;
                }
            }
        });
    }

    let mut primes: Arc<vec::Vec<u64>> = Arc::new(vec![2]);
    let mut squares: Arc<vec::Vec<u64>> = Arc::new(vec![4]);
    let mut pending: vec::Vec<Report> = vec::Vec::new();

    let mut top: u64 = 2;
    let mut top_quad: u64 = 4;
    let mut last_batch = Batch {
        primes: primes.clone(),
        squares: squares.clone(),
        start: 1,
        end: 3,
    };

    let mut do_stuff = || {
        let mut didwork = false;
        let chunk = if top >= batch { batch } else { top };

        for i in 0..working.len() {
            if !working[i] {
                let next_end = last_batch.end + chunk;
                if next_end - 1 <= top_quad {
                    didwork = true;
                    last_batch = Batch {
                        primes: primes.clone(),
                        squares: squares.clone(),
                        start: last_batch.end,
                        end: next_end,
                    };

                    channels[i].send(last_batch.clone()).unwrap();
                    working[i] = true;
                }
            }
        }

        match main_rx.try_recv() {
            Ok(report) => {
                working[report.index] = false;
                pending.push(report);
                didwork = true;
            }
            Err(_) => {
                // do nothing
            }
        }

        let mut consumed = true;
        while consumed {
            consumed = false;
            pending.retain(|report| {
                let connects = report.start - 1 == top;
                if connects {
                    didwork = true;
                    consumed = true;
                    primes = Arc::new(
                        primes
                            .iter()
                            .cloned()
                            .chain(report.primes.iter().cloned())
                            .collect(),
                    );
                    squares = Arc::new(
                        squares
                            .iter()
                            .cloned()
                            .chain(report.squares.iter().cloned())
                            .collect(),
                    );
                    top = report.end - 1;
                    top_quad = top * top;
                }

                !connects
            });
        }

        didwork
    };

    let now = SystemTime::now();

    while match now.elapsed() {
        Ok(e) => e.as_secs() < 60,
        Err(e) => {
            eprint!("{}", e);
            false
        }
    } {
        do_stuff();
        // nothing was done take small pause
        // thread::yield_now();
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
