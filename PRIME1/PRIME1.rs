use std::io;
use std::cmp::max;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn read_ints() -> Vec<i64> {
    let line = read_line();
    return line.split(" ")
        .filter(|&s| s.len() > 0)
        .map(|s| {
            s.parse::<i64>().unwrap()
        }).collect();
}

fn main() {
    let mut primes = vec![2i64, 3, 5, 7, 11, 13, 17, 19];
    for i in 11i64..20000 {
        let t = (i << 1) | 1;
        let mut j = 0;
        let mut is_prime = true;
        while primes[j] * primes[j] <= t {
            if t % primes[j] == 0 {
                is_prime = false;
                break;
            }
            j += 1;
        }
        if is_prime {
            primes.push(t);
        }
    }

    let params = read_ints();
    let t = params[0];
    for test_case in 0..t {
        let params = read_ints();
        let a = params[0];
        let b = params[1];
        for i in max(a, 2)..b + 1 {
            let mut j = 0;
            let mut is_prime = true;
            while primes[j] * primes[j] <= i {
                if i % primes[j] == 0 {
                    is_prime = false;
                    break;
                }
                j += 1;
            }
            if is_prime {
                println!("{}", i);
            }
        }
        if test_case != t - 1 {
            println!();
        }
    }
}