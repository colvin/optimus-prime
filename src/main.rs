extern crate clap;
use clap::{App, Arg};

struct Primer {
    v: u32,
}

impl Primer {
    fn new() -> Primer {
        Primer { v: 0 }
    }
}

impl Iterator for Primer {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.v {
            0 => self.v = 2,
            2 => self.v = 3,
            _ => {
                match (self.v + 2..)
                    .step_by(2)
                    .take_while(|x| !is_prime(*x))
                    .last()
                {
                    Some(n) => self.v = n + 2,
                    None => self.v += 2,
                }
            }
        }
        Some(self.v)
    }
}

fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i = i + 6;
    }
    true
}

fn main() {
    let args = App::new("optimus-prime")
        .version(env!("CARGO_PKG_VERSION"))
        .author("colvin")
        .about("prime number generator")
        .arg(
            Arg::with_name("nth")
                .short("n")
                .long("nth")
                .takes_value(true)
                .value_name("INDEX")
                .conflicts_with("count")
                .help("produce the nth prime, zero indexed"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .takes_value(true)
                .value_name("COUNT")
                .conflicts_with("nth")
                .help("produce a specific number of primes"),
        )
        .get_matches();

    let mut p = Primer::new();

    if let Some(s) = args.value_of("nth") {
        let n = s.parse::<usize>().unwrap();
        println!("{}", p.nth(n).unwrap());
    } else if let Some(s) = args.value_of("count") {
        let n = s.parse::<usize>().unwrap();
        for v in p.take(n) {
            println!("{}", v);
        }
    } else {
        for v in p {
            println!("{}", v);
        }
    }
}
