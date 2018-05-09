/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut sieve = vec![true; n as usize];
    for i in 2..((n as f64).sqrt() as u32) + 1 {
        if sieve[i as usize] {
            for k in 0.. {
                let mut j = i * i + k * i;
                if j >= n {
                    break
                }
                sieve[j as usize] = false;
            }
        }
    }
    let mut result = vec![];
    for i in 2..n {
        if sieve[i as usize] {
            result.push(i as u32);
        }
    }
    result
}