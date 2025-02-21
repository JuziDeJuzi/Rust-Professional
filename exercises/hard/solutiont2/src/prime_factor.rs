fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn pollard_rho(n: u128) -> u128 {
    if n == 1 { return 1; }
    if n % 2 == 0 { return 2; }
    if miller_rabin(n) { return n; }

    let f = |x: u128| -> u128 {
        let product = (x as u128).wrapping_mul(x as u128);
        (product.wrapping_add(1)) % n
    };

    let mut x = 2;
    let mut y = 2;
    let mut d = 1;

    while d == 1 {
        x = f(x);
        y = f(f(y));
        let abs_diff = if x >= y { x - y } else { y - x };
        d = gcd(abs_diff, n);
    }

    if d == n {
        return pollard_rho(n);
    }

    let factor1 = pollard_rho(d);
    let factor2 = pollard_rho(n / d);
    factor1.max(factor2)
}

fn miller_rabin(n: u128) -> bool {
    if n <= 1 || n == 4 { return false; }
    if n <= 3 { return true; }

    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    'next_base: for &a in bases.iter().take_while(|&&x| x < n) {
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }

        for _ in 0..r - 1 {
            x = (x as u128 * x as u128) % n;
            if x == n - 1 {
                continue 'next_base;
            }
        }
        return false;
    }
    true
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 { return 0; }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    result
}

pub fn find_max_prime_factor(mut n: u128) -> u128 {
    if n <= 1 { return n; }

    // 快速处理小因子
    let mut max_factor = 1;
    while n % 2 == 0 {
        max_factor = 2;
        n /= 2;
    }

    for i in (3..=1000).step_by(2) {
        while n % i == 0 {
            max_factor = i;
            n /= i;
        }
        if n == 1 { break; }
    }

    if n > 1 {
        // 对于较大的数使用 Pollard's Rho
        let factor = pollard_rho(n);
        max_factor = max_factor.max(factor);
    }

    max_factor
}