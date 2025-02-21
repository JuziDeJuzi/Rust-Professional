pub fn goldbach_conjecture() -> String {
    // 先生成足够大范围内的素数表
    let limit = 10000;
    let primes = sieve_of_eratosthenes(limit);

    // 存储找到的不符合猜想的数
    let mut results = Vec::new();

    // 从3开始检查奇合数
    let mut n = 3;
    while results.len() < 2 {
        if n % 2 == 1 && !is_prime(n) {  // 只检查奇合数
            if !can_be_represented(n, &primes) {
                results.push(n);
            }
        }
        n += 2;
    }

    format!("{},{}", results[0], results[1])
}

// 埃氏筛法生成素数表
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();

    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=(n as f64).sqrt() as usize {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
        }
    }

    primes
}

// 判断一个数是否为素数
fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt = (n as f64).sqrt() as i32;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// 检查一个数是否可以表示为一个素数加上一个平方数的两倍
fn can_be_represented(n: i32, primes: &[usize]) -> bool {
    let sqrt = (n as f64).sqrt() as i32;

    for &p in primes {
        if p as i32 > n {
            break;
        }
        // n = p + 2k^2
        // 2k^2 = n - p
        let diff = n - p as i32;
        if diff <= 0 {
            continue;
        }
        if diff % 2 != 0 {
            continue;
        }
        let k_squared = diff / 2;
        let k = (k_squared as f64).sqrt() as i32;
        if k * k == k_squared {
            return true;
        }
    }
    false
}