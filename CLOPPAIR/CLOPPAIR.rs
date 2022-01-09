use std::collections::HashMap;
use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn read_ints() -> Vec<i64> {
    let line = read_line();
    return line
        .split(" ")
        .filter(|&s| s.len() > 0)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
}

fn main() {
    let n: usize = read_ints()[0] as usize;
    let mut points: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        let p = read_ints();
        points.push((p[0], p[1]));
    }

    let mut lo: f64 = 0.0;
    let mut hi: f64 = 4e6;
    let mut buckets: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
    let neighbors = [
        (0, 0),
        (-1, 0),
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let mut closest: (usize, usize) = (0, 0);
    let mut valid: (usize, usize);
    while hi - lo >= 2e-7 {
        buckets.clear();
        valid = (0, 0);
        let mid = (lo + hi) / 2.0;
        for i in 0..n {
            let x = (points[i].0 as f64 / mid).floor() as i64;
            let y = (points[i].1 as f64 / mid).floor() as i64;
            for &(dx, dy) in neighbors.iter() {
                let xp = x + dx;
                let yp = y + dy;

                if !buckets.contains_key(&(xp, yp)) {
                    continue;
                }

                for &j in buckets.get(&(xp, yp)).unwrap() {
                    if ((points[i].0 - points[j].0) * (points[i].0 - points[j].0)
                        + (points[i].1 - points[j].1) * (points[i].1 - points[j].1))
                        as f64
                        <= mid * mid
                    {
                        valid = (j, i);
                        closest = (j, i);
                        break;
                    }
                }

                if valid != (0, 0) {
                    break;
                }
            }

            if valid != (0, 0) {
                break;
            }

            (*buckets.entry((x, y)).or_default()).push(i);
        }

        if valid == (0, 0) {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    let (i, j) = closest;
    println!("{} {} {:.6}", i, j, lo);
}
