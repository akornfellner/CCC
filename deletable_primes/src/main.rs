use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].to_string();

    println!("{}", deletable_primes(&n));
}

fn is_prime(n: usize) -> bool {
    if n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn get_subs(n: &str) -> Vec<String> {
    let mut subs = Vec::new();
    for i in 0..n.len() {
        let mut s = n.to_string();
        s.remove(i);
        subs.push(s);
    }
    subs
}

fn deletable_primes(n: &str) -> usize {
    let mut count = 0;
    let subs = get_subs(n);

    for s in subs {
        if is_prime(s.parse().unwrap()) {
            if s.len() > 1 {
                count += deletable_primes(&s);
            } else {
                count += 1;
            }
        }
    }
    count
}
