/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut matrix = vec![vec![1, 1], vec![1, 0]]; // 基本的斐波那契矩阵
    let result_matrix = matrix_pow(matrix, n - 1); // 计算矩阵的 (n-1) 次幂
    result_matrix[0][0] // F(n) 就是矩阵的 [0][0] 元素
}
// 矩阵快速幂运算
fn matrix_pow(mut matrix: Vec<Vec<i32>>, mut power: i32) -> Vec<Vec<i32>> {
    let mut result = vec![vec![1, 0], vec![0, 1]]; // 单位矩阵
    while power > 0 {
        if power % 2 == 1 {
            result = matrix_multiply(result, matrix.clone());
        }
        matrix = matrix_multiply(matrix.clone(), matrix.clone());
        power /= 2;
    }
    result
}

fn matrix_multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![0, 0], vec![0, 0]];
    for i in 0..2 {
        for j in 0..2 {
            result[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
